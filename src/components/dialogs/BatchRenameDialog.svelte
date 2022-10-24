<script lang="ts">
    import { createEventDispatcher } from "svelte"
    import Dialog from "../utils/Dialog.svelte"
    export let open = true

    let from = ""
    let to = ""

    let dispatch = createEventDispatcher()

    let closeDialog = () => {
        from = ""
        to = ""
        open = false
    }

    let confirmRename = () => {
        dispatch("confirm", {from, to})
        closeDialog()
    }
</script>

<Dialog {open} on:close={() => closeDialog}>
    <form>
        <label for="rename-from-input" class="label"><span class="label-text">from</span></label>
        <input id="rename-from-input" bind:value={from} type="text" class="input input-bordered w-full max-w-xs">
        <label for="rename-to-input" class="label"><span class="label-text">to</span></label>
        <input id="rename-to-input" bind:value={to} type="text" class="input input-bordered w-full max-w-xs">
        <div class="modal-action">
            <label for="my-modal" class="btn btn-ghost" on:click={() => closeDialog()}>Cancel</label>
            <label for="my-modal" class="btn" on:click={() => confirmRename()}>Rename</label>
        </div>
    </form>
</Dialog>

<style>
    label.btn-ghost {
        color: hsl(var(--nc) / var(--tw-text-opacity));
    }
</style>