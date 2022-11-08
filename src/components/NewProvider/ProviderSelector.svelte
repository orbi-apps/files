<script lang="ts">
    import { Providers } from "src/api/Providers"
    import { createEventDispatcher } from "svelte"

    const dispatch = createEventDispatcher()

    const providers = [
        { name: "Database", type: Providers.SQL, logo: "/images/database.png" },
        { name: "Google Drive", type: Providers.GoogleDrive, logo: "https://upload.wikimedia.org/wikipedia/commons/d/da/Google_Drive_logo.png" },
        { name: "Local Drive", type: Providers.Native, logo: "/images/drive.png" },
        { name: "S3", type: Providers.S3, logo: "https://cdn.freebiesupply.com/logos/large/2x/aws-s3-logo-png-transparent.png" },
    ]

    let filter = ""

    $: filteredProviders = providers.filter(provider => {
        return provider.name.toLowerCase().includes(filter.toLowerCase())
    })
</script>

<div class="text-center prose">
    <h2>Select a provider</h2>
    <input bind:value={filter} class="input input-bordered" type="text" placeholder="Search..." />
    <div class="grid grid-cols-4 gap-4 mt-5">
        {#each filteredProviders as provider}
            <button class="btn flex flex-col justify-around w-full h-full not-prose" on:click={() => dispatch("selected", provider.type)}>
                <figure>
                    <img src={provider.logo} alt={provider.name} />
                </figure>
                <p class="mx-1">
                    {provider.name}
                </p>
            </button>
        {/each}
    </div>
</div>

<style>
    figure {
        margin: auto;
        max-width: 3rem;
        max-height: 3rem;
    }
    .btn {
        height: 7rem;
        padding: 0;
        padding-bottom: 0.5rem;
        padding-top: 0;
        font-weight: 500;
    }
</style>