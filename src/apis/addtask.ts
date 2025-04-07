import { invoke } from "@tauri-apps/api/core";
export async function addtask(name: string): Promise<number> {
  const taskid = await invoke<number>("add_task", {
    name: name,
    isDone: false,
  });
  return taskid;
}
