<script lang="ts">
    import { mdiFolderPlusOutline, mdiChevronRight, mdiLanguageMarkdownOutline, mdiFileDocumentPlusOutline, mdiLanguagePython, mdiFileDelimitedOutline } from "@mdi/js"
    import { createEventDispatcher } from "svelte"
    import { vfs } from "../../store"
    import Icon from "./Icon.svelte"
    import filesTemplates, { type FileTemplate } from "./files.templates"

    export let position: {x: number, y:number} = {x: 0, y: 0}
    export let open: boolean

    const dispatchCreate = createEventDispatcher<{create: FileTemplate}>()
    const dispatch = createEventDispatcher()

    let mouseLeaveTimeout: NodeJS.Timeout

    $: y = window.innerHeight * 2 / 3 > position.y ? `top: ${position.y}px;` : `bottom: ${window.innerHeight - position.y}px;`
    $: x = window.innerWidth * 2 / 3 > position.x ? `left: ${position.x}px;` : `right: ${window.innerWidth - position.x}px;`

    const onMouseLeaveMenu = () => {
        mouseLeaveTimeout = setTimeout(() => {
            open=false
        }, 100)
    }

    const OnMouseEnterMenu = () => {
        clearTimeout(mouseLeaveTimeout)
    }
</script>

{#if open}
<ul class="menu bg-base-100 w-56 shadow" style={`position: fixed; ${y} ${x} z-index:999`} on:mouseleave={onMouseLeaveMenu} on:mouseenter={OnMouseEnterMenu}>
    <li tabindex="0">
        <div class="flex justify-between flex-row">
            <span>New</span><Icon icon={mdiChevronRight}></Icon>
        </div>
        <ul class="bg-base-100 w-48 shadow">
            <li><button on:click={() => dispatchCreate('create', filesTemplates.directory)}><Icon icon={mdiFolderPlusOutline}></Icon>Folder</button></li>
            <li><button on:click={() => dispatchCreate('create', filesTemplates.emptyFile)}><Icon icon={mdiFileDocumentPlusOutline}></Icon>Empty file</button></li>
            <li class="menu-title">
                <span><hr /></span>
            </li>
            <li><button on:click={() => dispatchCreate('create', filesTemplates.markdown)}><Icon icon={mdiLanguageMarkdownOutline}></Icon>Markdown</button></li>
            <li><button on:click={() => dispatchCreate('create', filesTemplates.python)}><Icon icon={mdiLanguagePython}></Icon>Python</button></li>
            <li><button on:click={() => dispatchCreate('create', filesTemplates.csv)}><Icon icon={mdiFileDelimitedOutline}></Icon>CSV</button></li>
        </ul>
    </li>
    <li><button on:click={(event) => {event.stopPropagation();$vfs.open(); open = false}}>Open</button></li>
    <li><button on:click={(event) => {event.stopPropagation();$vfs.open()}}>Open in browser</button></li>
    <!-- <li><button on:click={(event) => {event.stopPropagation();$vfs.open()}}>Download</button></li> -->
    <li><button on:click={(event) => {event.stopPropagation();dispatch('rename')}}>Rename</button></li>
    <!-- <li><button on:click={(event) => {event.stopPropagation();dispatch('move')}}>Move</button></li> -->
    <li><button on:click={(event) => {event.stopPropagation();$vfs.delete(); open = false}}>Delete</button></li>
</ul>
{/if}