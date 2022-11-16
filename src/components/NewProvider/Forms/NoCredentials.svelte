<script lang="ts">
    import { createEventDispatcher } from 'svelte'

    const dispatch = createEventDispatcher()
</script>

<button class="btn" on:click={() => dispatch("submit")}>Login</button>