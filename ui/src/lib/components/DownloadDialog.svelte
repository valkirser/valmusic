<script lang="ts">
	import { onMount } from 'svelte';
	import { open as pickFolder } from '@tauri-apps/plugin-dialog';
	import { Button } from './ui/button';
	import * as Dialog from './ui/dialog';
	import * as api from '$lib/api';
	import type { SongItem, DownloadProgress } from '$lib/api';
	import { toast } from '$lib/player.svelte';
	import { t } from '$lib/i18n.svelte';

	let {
		open = $bindable(false),
		songs,
		playlistName,
		playlistId,
		archive = false,
		continuation,
		onClose
	}: {
		open?: boolean;
		songs: SongItem[];
		playlistName?: string;
		playlistId?: string;
		archive?: boolean;
		continuation?: string;
		onClose?: () => void;
	} = $props();
	let folder = $state('');
	let format = $state('mp3');
	let busy = $state(false);
	let progress = $state<DownloadProgress>({ done: 0, total: 0, current: '', progress: 0 });

	function close() {
		open = false;
		onClose?.();
	}
	function handleOpenChange(value: boolean) {
		open = value;
		if (!value) onClose?.();
	}

	onMount(() => {
		let unlisten: (() => void) | undefined;
		void api.onDownloadProgress((value) => (progress = value)).then((stop) => (unlisten = stop));
		return () => unlisten?.();
	});

	async function chooseFolder() {
		const selected = await pickFolder({ directory: true, multiple: false, title: t('dialogs.download.choose_folder_title') });
		if (typeof selected === 'string') folder = selected;
	}

	function friendlyPath(path: string): string {
		const extendedUnc = '\\\\?\\UNC\\';
		const extended = '\\\\?\\';
		if (path.startsWith(extendedUnc)) return `\\\\${path.slice(extendedUnc.length)}`;
		return path.startsWith(extended) ? path.slice(extended.length) : path;
	}

	async function start() {
		if (!folder || busy || songs.length === 0) return;
		busy = true;
		progress = { done: 0, total: songs.length, current: songs[0]?.title ?? '', progress: 0 };
		try {
			const selected = [...songs];
			let token = continuation;
			while (archive && token) {
				progress = { done: 0, total: selected.length, current: t('dialogs.download.loading_playlist'), progress: 0 };
				const page = await api.getPlaylistMore(token, playlistId);
				selected.push(...page.items);
				token = page.continuation;
			}
			const path = await api.downloadSongs(selected, folder, format, playlistName, archive);
			toast.success(t('dialogs.download.saved', { path: friendlyPath(path) }));
			close();
		} catch (error) {
			toast.error(String(error));
		} finally {
			busy = false;
		}
	}
</script>


<Dialog.Root
	open={open}
	onOpenChange={handleOpenChange}
>
	<Dialog.Content showCloseButton={false} class="max-w-lg gap-0 rounded-xl border bg-card p-5 text-card-foreground shadow-2xl">
		<section class="w-full space-y-5">
			<header>
				<Dialog.Title class="text-lg font-semibold">{t('dialogs.download.title')}</Dialog.Title>
				<Dialog.Description>
					{t(songs.length === 1 ? 'dialogs.download.one_song' : 'dialogs.download.many_songs', { count: songs.length, archive_note: archive ? t('dialogs.download.archive_note') : '' })}
				</Dialog.Description>
			</header>
			<div class="space-y-2">
				<label class="block text-sm font-medium" for="download-folder">{t('dialogs.download.folder')}</label>
				<div class="flex gap-2">
					<input id="download-folder" readonly value={folder} placeholder={t('dialogs.download.choose_folder')} class="min-w-0 flex-1 rounded-md border bg-background px-3 py-2 text-sm" />
					<Button variant="outline" onclick={chooseFolder}>{t('dialogs.download.browse')}</Button>
				</div>
			</div>
			<div class="space-y-2">
				<label class="block text-sm font-medium" for="download-format">{t('dialogs.download.format')}</label>
				<select id="download-format" bind:value={format} class="w-full rounded-md border bg-background px-3 py-2 text-sm" disabled={busy}>
					<option value="original">{t('dialogs.download.original')}</option>
					<option value="mp3">MP3</option>
					<option value="m4a">M4A (AAC)</option>
					<option value="opus">Opus</option>
					<option value="flac">FLAC</option>
					<option value="wav">WAV</option>
				</select>
			</div>
			{#if busy}
				<div class="space-y-2" aria-live="polite">
					<div class="flex justify-between gap-3 text-sm"><span class="truncate">{progress.current || t('dialogs.download.preparing')}</span><span class="shrink-0">{progress.done}/{progress.total || songs.length}</span></div>
					<progress class="h-2 w-full accent-primary" max="1" value={progress.progress}></progress>
				</div>
			{/if}
			<footer class="flex justify-end gap-2">
				<Button variant="ghost" disabled={busy} onclick={close}>{t('common.cancel')}</Button>
				<Button disabled={busy || !folder || songs.length === 0} onclick={start}>{busy ? t('dialogs.download.downloading') : t('dialogs.download.download')}</Button>
			</footer>
		</section>
	</Dialog.Content>
</Dialog.Root>
