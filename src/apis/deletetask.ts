import { invoke } from "@tauri-apps/api/core";
import type { Task } from "../components/task.svelte";

export async function deletetask(task: Task) {
  await invoke("delete_task", { id: task.id });
}
