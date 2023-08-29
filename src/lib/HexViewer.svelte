<script lang="ts">
    export let array: number[];
    export let bound: "Byte" | "Short" | "Int" | "Long" | "Float" | "Double";
    let isOpen = false;

    let dialog: HTMLDialogElement;

    export function close() {
        dialog.close();
        isOpen = false;
    }

    export function open() {
        dialog.showModal();
        isOpen = true;
    }
</script>

<dialog class="modal" bind:this={dialog}>
    <form class="modal-box">
        <div class="flex flex-wrap">
            {#if isOpen}
                {#each array as d}
                    <input type="number" inputmode="decimal" value={d} />
                {/each}
            {/if}
        </div>
        <div class="modal-action">
            <button class="btn" on:click={close}>Cancel</button>
            <!--
            <button class="btn btn-primary" on:click={close}>Save</button>
            -->
        </div>
    </form>
    <form method="dialog" class="modal-backdrop" on:submit|preventDefault={close}>
        <button>close</button>
    </form>
</dialog>
