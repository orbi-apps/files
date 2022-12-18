<script lang="ts">
  import type { ProviderId } from 'src/api/Providers';
  import { vfs } from '../store'
  import MainFileList from './MainFileList.svelte';
  import ProvidersSelector from './ProvidersList.svelte'

  let activeProvider: ProviderId | undefined = $vfs.activeProvider

  $vfs.register(() => {
    activeProvider = $vfs.activeProvider
  })
</script>

<div>
  {#if activeProvider}
    <MainFileList />
  {:else}
    <ProvidersSelector />
  {/if}
</div>



<style>
  div {
    overflow: hidden;
  }
</style>