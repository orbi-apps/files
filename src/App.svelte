<script lang="ts">
    import Main from './components/Main.svelte'
    import ToolBar from './components/ToolBar.svelte'
    import "./app.css"
    import { listen, emit } from '@tauri-apps/api/event'
    import { appWindow } from '@tauri-apps/api/window'
    import { settings } from './store'

    listen("windowSettings", (event) => {
        settings.set(JSON.parse(event.payload as string))
    })

    appWindow.onCloseRequested(() => {
        emit("closeDialog")
    })

    emit("ready")
</script>

<div class="card shadow-lg">
    <ToolBar />
    <Main />
</div>

<style>
    :global(body) {
        margin: 0;
    }

    .card {
        height: calc(100vh - 18px);
        width: calc(100vw - 18px);
        margin: auto;
        background-color: hsla(var(--b1) / var(--tw-bg-opacity, 1));
    }
</style>
