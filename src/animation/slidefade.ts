import type { TransitionConfig } from "svelte/transition";
import { backOut } from "svelte/easing";
//animation use with svelte transistion
export function slideFade(
  node: HTMLElement,
  { duration = 500, index = 0, stagger = 0.1 } = {},
): TransitionConfig {
  return {
    delay: index * stagger,
    duration,
    css: (t, u) => `
      opacity: ${1 - t};
      transform: translatey(${u * 20}px);
    `,
    easing: backOut,
  };
}
