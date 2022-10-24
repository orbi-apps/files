<script lang="ts">
    /**
     * @type {string}
     */
    export let icon: string
    export let size: number = 24
    export let color: string = 'hsla(var(--bc) / var(--tw-text-opacity, 1))'
    export let style: string = ''

    let scale = size/24
</script>
  
<svg style="width:{size}px;height:{size}px;{style}">
    <path fill={color} d={icon} transform="scale({scale})" />
</svg>
  