<script lang="ts">
    import { mdiChevronLeft, mdiCheck } from '@mdi/js'
    import { createEventDispatcher } from 'svelte'
    import Icon from "../../utils/Icon.svelte"

    const dispatch = createEventDispatcher()

    let bucket = ""
    let region = ""
    let endpoint = ""
    let access_key = ""
    let secret_key = ""

    const onBack = (event: { preventDefault: () => void; }) => {
        event.preventDefault()
        dispatch("back")
    }

    const onSubmit = (event: Event & { currentTarget: EventTarget & HTMLFormElement}) => {
        event.preventDefault()

        dispatch("submit", { name: bucket, credentials: { bucket, credentials : { region, endpoint, access_key, secret_key } } })
    }
</script>

<form class="prose" id="provider-credentials-form" on:reset={event => event.preventDefault()} on:submit={onSubmit}>
    <div class="form-control">
        <label for="bucket" class="label label-text">Bucket name</label>
        <input name="bucket" class="input input-bordered w-full" bind:value={bucket} />
    </div>
    <div class="form-control">
        <label for="region" class="label label-text">Region</label>
        <input name="region" class="input input-bordered w-full" bind:value={region} />
    </div>
    <div class="form-control">
        <label for="endpoint" class="label label-text">Endpoint</label>
        <input name="endpoint" class="input input-bordered w-full" bind:value={endpoint} />
    </div>
    <div class="form-control">
        <label for="access_key" class="label label-text">Access Key</label>
        <input name="access_key" class="input input-bordered w-full" bind:value={access_key} />
    </div>
    <div class="form-control">
        <label for="secret_key" class="label label-text">Secret Key</label>
        <input name="secret_key" class="input input-bordered w-full" bind:value={secret_key} />
    </div>
    <div class="flex justify-around place-items-center mt-5">
        <button type="reset" class="btn"><Icon icon={mdiChevronLeft}/></button>
        <button type="submit" class="btn" form="provider-credentials-form"><Icon icon={mdiCheck} /></button>
    </div>
</form>
