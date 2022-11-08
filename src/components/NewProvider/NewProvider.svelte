<script lang="ts">
    import { addProvider } from "@junction"
    import { ObjectId } from "src/api/object"
    import { Providers, type ProviderId } from "src/api/Providers"
    import { createEventDispatcher } from "svelte"
    import Dialog from "../utils/Dialog.svelte"
    import S3Form from "./Forms/S3Form.svelte"
    import ProviderSelector from "./ProviderSelector.svelte"

    export let open: boolean
    let step = 0

    const dispatch = createEventDispatcher()

    let id: ProviderId
    let name = ""
    let type: Providers

    const closeDialog = () => {
        dispatch("close")
    }

    const onSubmitProviderCredentials = (nameFromForm: string, data: any) => {
        console.log(data)
        id = {
            id: nameFromForm,
            type,
            data
        }

        name = id.id

        step++
    }

    const confirmNewProvider = (event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement; }) => {
        event.preventDefault()
        id.id = name
        addProvider(id, id.data)
        closeDialog()
    }
</script>

<Dialog {open} on:close={() => closeDialog()}>
    {#if step == 0}
        <ProviderSelector on:selected={event => { step++; type = event.detail }} />
    {:else if step == 1}
        {#if type == Providers.GoogleDrive}
            <div></div>
        {:else if type == Providers.S3}
            <S3Form on:submit={event => onSubmitProviderCredentials(event.detail.name, event.detail.credentials)} />
        {:else if type == Providers.SQL}
            <div></div>
        {:else if type == Providers.Native}
            <div></div>
        {:else}
            <div>Unsupported Provider</div>
        {/if}
    {:else}
        <form style="color: hsl(var(--pc));" on:submit={(event) => confirmNewProvider(event)} on:reset={event => {event.preventDefault(); closeDialog()}}>
            <div class="form-control">
                <label for="provider-name-input" class="label label-text">Namae</label>
                <input id="provider-name-input" bind:value={name} type="text" class="input input-bordered w-full">
            </div>
            
            <div class="modal-action">
                <input type="reset" class="btn btn-ghost" value="Cancel" />
                <input type="submit" class="btn" value="Confirm" />
            </div>
        </form>
    {/if}
    <div class="flex align-center mt-5">
        <ul class="steps m-auto">
            <li class="step prose" class:step-primary={step >= 0}>Select provider</li>
            <li class="step prose" class:step-primary={step >= 1}>Login</li>
            <li class="step prose" class:step-primary={step >= 2}>Configure</li>
        </ul>
    </div>
</Dialog>