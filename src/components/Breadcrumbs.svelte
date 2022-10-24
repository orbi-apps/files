<script lang="ts">
    import { get } from "svelte/store"
    import { vfs } from "../store"
    import { mdiHomeOutline } from '@mdi/js'
    import Icon from "./utils/Icon.svelte"
    
    let values = $vfs.pathAsArray() || []

    $vfs.register(() => {
        values = get(vfs)?.pathAsArray() || []
    })
</script>

<div class="text-sm breadcrumbs">
    <ul>
        <li><button class="btn btn-sm btn-ghost" on:click={(e) => { e.stopPropagation(); $vfs.gotoIndex(0) }}><Icon icon={mdiHomeOutline} /></button></li>
        {#each values as value, index}
            <li>
                <button class="btn btn-sm btn-ghost" on:click={(e) => { e.stopPropagation(); $vfs.gotoIndex(index + 1) }}>
                    {value.name}
                </button>
            </li>
        {/each}
    </ul>
</div>

<style>
    .breadcrumbs {
        min-width: 100px;
        margin: auto 15px;
        flex-grow: 1;
        height: 48px;
        line-height: 48px;
    }

    .breadcrumbs ul {
        display: flex;
        justify-content: center;
    }

    button.btn {
        border-radius: var(--rounded-btn, 0.5rem);
        text-transform: none;
        font-size: inherit;
        text-decoration: none;
        padding-left: 0.2rem;
        padding-right: 0.2rem;
        font-weight: 400;
    }
</style>