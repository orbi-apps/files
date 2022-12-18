<script lang="ts">
    import { Providers, type ProviderId } from "src/api/Providers"
    import { createEventDispatcher } from "svelte"
    import Dialog from "../utils/Dialog.svelte"
    import NoCredentials from "./Forms/NoCredentials.svelte"
    import S3Form from "./Forms/S3Form.svelte"
    import ProviderSelector from "./ProviderSelector.svelte"
    import { vfs } from "src/store"

    export let open: boolean
    let step = 0

    const dispatch = createEventDispatcher()

    let id: ProviderId
    let name = ""
    let type: Providers

    const closeDialog = () => {
        dispatch("close")
    }

    const onSubmitProviderCredentials = async (data?: any) => {
        id.data = data || {}

        await $vfs.addProvider(id, id.data)
        closeDialog()
    }

    const confirmNewProvider = (event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement; }) => {
        event.preventDefault()
        id = {
            id: name,
            type,
            data: {}
        }

        step++
    }
</script>

<Dialog {open} on:close={() => closeDialog()}>
    {#if step == 0}
        <ProviderSelector on:selected={event => { step++; type = event.detail }} />
    {:else if step == 1}
        <form style="color: hsl(var(--pc));" on:submit={(event) => confirmNewProvider(event)} on:reset={event => {event.preventDefault(); step--}}>
            <div class="form-control">
                <label for="provider-name-input" class="label label-text">Name</label>
                <input id="provider-name-input" bind:value={name} type="text" class="input input-bordered w-full">
            </div>
            
            <div class="modal-action">
                <input type="reset" class="btn btn-ghost" value="Back" />
                <input type="submit" class="btn" value="Next" />
            </div>
        </form>
    {:else}
        {#if type == Providers.GoogleDrive}
            <NoCredentials on:submit={() => onSubmitProviderCredentials()} />
        {:else if type == Providers.OneDrive}
            <NoCredentials on:submit={() => onSubmitProviderCredentials()} />
        {:else if type == Providers.S3}
            <S3Form on:submit={event => onSubmitProviderCredentials(event.detail.credentials)} />
        {:else if type == Providers.SQL}
            <div></div>
        {:else if type == Providers.Native}
            <div></div>
        {:else}
            <div>Unsupported Provider</div>
        {/if}
    {/if}
    <div class="flex align-center mt-5">
        <ul class="steps m-auto">
            <li class="step prose" class:step-primary={step >= 0} on:click={event => {event.stopPropagation(); step = 0}}>Select provider</li>
            <li class="step prose" class:step-primary={step >= 1} on:click={event => {event.stopPropagation(); step = 1}}>Configure</li>
            <li class="step prose" class:step-primary={step >= 2} on:click={event => {event.stopPropagation(); step = 2}}>Login</li>
        </ul>
    </div>
</Dialog>

<style>
    li.step {
        cursor: pointer;
    }
</style>