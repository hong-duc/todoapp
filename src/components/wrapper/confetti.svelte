<script lang="ts">
  import Confetti from "svelte-confetti";
  import { onMount } from "svelte";

  let { children } = $props();
  let showconfetti = $state(false);
  let wrapperelement: HTMLElement;
  let confettiPosition = $state({ x: 0, y: 0 });
  // Expose a function to toggle confetti
  export function triggerConfetti() {
    updatePosition();
    showconfetti = true;
    setTimeout(() => (showconfetti = false), 2000);
  }
  function updatePosition() {
    if (wrapperelement) {
      const rect = wrapperelement.getBoundingClientRect();
      confettiPosition = {
        x: rect.left + rect.width / 2,
        y: rect.top,
      };
    }
  }

  onMount(() => {
    updatePosition();
  });
</script>

<div bind:this={wrapperelement} class="confetti-wrapper">
  {@render children(triggerConfetti)}

  <!-- Confetti appears at the slot's position -->
  {#if showconfetti}
    <div
      class="confetti-overlay"
      style="left: {confettiPosition.x}px; top: {confettiPosition.y}px"
    >
      <Confetti
        amount={100}
        size={6}
        duration={2000}
        colorArray={["#FF5252", "#FFD740", "#64FFDA"]}
      />
    </div>
  {/if}
</div>
