<script lang="ts">
    import { addProvider } from '@junction'
    import type { ProviderId } from 'src/api/Providers'
    import { onMount } from 'svelte'
    import { vfs } from '../store'
    import NewProvider from './NewProvider/NewProvider.svelte'

    let services = []

    let newProviderDialogOpen = false

    onMount(async () => {
        services = $vfs.providers || []
    })

    $vfs.register(() => {
        services = $vfs.providers || []
    })
  
    const goto = (provider: ProviderId) => {
        $vfs.setProvider(provider)
    }

    const newProvider = () => {
        newProviderDialogOpen = true
    }
</script>

<NewProvider open={newProviderDialogOpen} on:close={() => newProviderDialogOpen = false} />
  
<div class="overflow-x-auto table-auto disable-select h-full">
    <table class="table w-full overflow-y-scroll">
        <tbody>
            {#each services as service}
                <tr class="hover" on:dblclick={() => goto(service)}>
                    <td>{service.id}</td>
                </tr>
            {/each}
            <tr class="hover">
                <td role="button" class="btn btn-ghost btn-block" on:click={newProvider}>
                    + New Provider
                </td>
            </tr>
        </tbody>
    </table>
</div>

<style>
    td.btn {
        justify-content: flex-start;
        font-weight: 400;
    }

    tbody {
        overflow-y: scroll;
    }
</style>