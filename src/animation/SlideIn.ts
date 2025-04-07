import { gsap } from "gsap";
export function SlideIn(node: HTMLElement, index: number) {
  gsap.from(node, {
    opacity: 0,
    y: 50,
    duration: 0.5,
    delay: index * 0.1,
    ease: "back.out",
  });
}
