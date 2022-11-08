<script lang="ts">
    import { createEventDispatcher } from "svelte"

    export let open: boolean
    let dialog: HTMLDialogElement

    const dispatch = createEventDispatcher()

    $: if (open && dialog && !dialog.open) {
        dialog.showModal()
    } else if (dialog?.open) {
        dialog.close()
    }

    let checkClickOutside = (event: { clientY: number; clientX: number; }) => {
        let rect = dialog.getBoundingClientRect();
        let isInDialog=(rect.top <= event.clientY && event.clientY <= rect.top + rect.height
          && rect.left <= event.clientX && event.clientX <= rect.left + rect.width);
        if (!isInDialog) {
            dialog.close()
        }

    }
</script>

<dialog
    bind:this={dialog}
    class="modal-box"
    on:close={() => dispatch("close")}
    on:click={checkClickOutside}>
    <slot />
</dialog>

<style>
    dialog::backdrop {
        background-color: rgba(0, 0, 0, 0.5);
    }
</style>