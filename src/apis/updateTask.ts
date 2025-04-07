import { invoke } from "@tauri-apps/api/core";
import type { Task } from "../components/task.svelte";
export function updateTask(task: Task) {
  return invoke("update_task", {
    id: task.id,
    isDone: task.is_done,
    name: task.name,
  }).catch((error) => {
  });
}
