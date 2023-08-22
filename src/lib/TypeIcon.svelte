<script lang="ts">
    export let char: string;
    export let color: string = "bg-green-500";
</script>

<div class="rounded-box text-black font-bold w-5 flex place-items-center {color}">
    <span class="text-center w-full">{char}</span>
</div>
