<script lang="ts">
  import Pencil from "../icons/pencil.svelte";
  import Trashcan from "../icons/trashcan.svelte";
  import WrapperConfetti from "./wrapper/confetti.svelte";
  import { onMount } from "svelte";

  export type Task = {
    id: string;
    name: string;
    is_done: boolean;
  };
  let {
    name,
    is_done,
    id,
    taskChange,
    ondelete,
  }: {
    name: string;
    is_done: boolean;
    id: string;
    taskChange: (task: Task) => void;
    ondelete: (task: Task) => void;
  } = $props();
  let isEditable = $state(false);
  let is_run_cancel_edit = $state(true);
  let old_value = name;
  onMount(() => {
    // console.log("mount localtask", JSON.stringify(localtask));
  });

  function cancelEdit() {
    name = old_value;
    isEditable = false;
  }
</script>

<div class="flex flox-row gap-1" id={`task-${id}`}>
  <WrapperConfetti>
    {#snippet children(triggerConfetti)}
      <input
        type="checkbox"
        class="checkbox checkbox-md checkbox-primary"
        bind:checked={is_done}
        onchange={(event) => {
          const node = event.target as HTMLInputElement;

          if (node.checked) {
            triggerConfetti();
          }
          taskChange({ id, is_done, name });
        }}
      />
    {/snippet}
  </WrapperConfetti>

  <div class="join">
    <input
      type="text"
      class="input join-item"
      bind:value={name}
      disabled={!isEditable}
      onblur={() => {
        if (is_run_cancel_edit) {
          cancelEdit();
        }
      }}
      onkeydown={(event) => {
        if (event.key === "Enter") {
          is_run_cancel_edit = false;
          taskChange({ id, is_done, name });
          isEditable = false;
        } else if (event.key === "Escape") {
          cancelEdit();
        }
      }}
    />
    <button
      class="btn join-item btn-neutral"
      onclick={() => {
        isEditable = true;
      }}
    >
      <Pencil />
    </button>
    <button
      class="btn join-item btn-error"
      onclick={() => ondelete({ id, is_done, name })}
    >
      <Trashcan />
    </button>
  </div>
</div>
