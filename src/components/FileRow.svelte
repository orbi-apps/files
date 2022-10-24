<script lang="ts">
    import { createEventDispatcher } from "svelte"
    import Icon from "./utils/Icon.svelte"
    import { mdiContentSave, mdiFolder, mdiFile } from "@mdi/js"
    import type { GenericObject } from "src/api/object"

    export let rename: boolean
    export let active: boolean
    export let file: GenericObject

    $: icon = file.id.mimeType.isFolder() ? mdiFolder : mdiFile

    const dispatch = createEventDispatcher()

    let newName = ""

    const resetNewName = (file: GenericObject) => {
        newName = file.name
    }
    $: resetNewName(file)

    const submit = (event) => {
        event.preventDefault()
        dispatch('rename', { oldName: file.name, newName })
    }
</script>

<tr
    class="hover"
    class:active={active}
    on:click
    on:dblclick
    on:contextmenu>
    <td class="file-icon"><Icon icon={icon} color="#fff" /></td>
    <td>
    {#if rename}
        <form class="form-control" on:submit={(event) => submit(event)}>
            <div class="input-group">
                <input class="input" autofocus bind:value={newName} />
                <button class="btn" type="submit"><Icon icon={mdiContentSave} /></button>
            </div>
        </form>
    {:else}
        {file.name}
    {/if}
    </td>
</tr>