<script lang="ts">
    import type { ProviderId } from 'src/api/Providers'
    import { onMount } from 'svelte'
    import { vfs } from '../store'

    let services = []

    onMount(async () => {
        services = $vfs.providers || []
    })

    $vfs.register(() => {
        services = $vfs.providers || []
    })
  
    const goto = (provider: ProviderId) => {
        $vfs.setProvider(provider)
    }
  </script>
  
  <div class="overflow-x-auto">
    <table class="table w-full">
      <!-- head -->
      <thead>
        <tr>
          <th></th>
          <th>Name</th>
          <th>Modified at</th>
          <th>Size</th>
        </tr>
      </thead>
      <tbody>
        {#each services as service}
          <tr class="hover" on:dblclick={() => goto(service)}>
            <td></td>
            <td>{service.id}</td>
            <td>Cy Ganderton</td>
            <td>Blue</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>