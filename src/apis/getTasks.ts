import { invoke } from "@tauri-apps/api/core";
import type { Task } from "../components/task.svelte";
export function getTasks(): Promise<Task[]> {
  return invoke("get_tasks");
}
