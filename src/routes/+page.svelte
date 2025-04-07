<script lang="ts">
  let ViewMode = {
    All: 1,
    done: 2,
    notDone: 3,
  };
  import { slideFade } from "../animation/slidefade";
  import { SlideIn } from "../animation/SlideIn";
  import { onMount } from "svelte";
  import Task from "../components/task.svelte";
  import type { Task as task } from "../components/task.svelte";
  import { deletetask } from "../apis/deletetask";
  import { getTasks } from "../apis/getTasks";
  import { addtask } from "../apis/addtask";
  import { updateTask } from "../apis/updateTask";
  let viewMode = $state(ViewMode.notDone);
  let tasks = $state<Array<task>>([
    { id: "1", name: "task1", is_done: true },
    { id: "2", name: "task2", is_done: false },
    { id: "3", name: "task3", is_done: false },
    { id: "4", name: "task4", is_done: true },
  ]);
  let viewTasks = $derived(
    viewMode === ViewMode.done
      ? tasks.filter((task) => {
          console.log("task in filter: ", JSON.stringify(task));
          return $state.snapshot(task).is_done;
        })
      : viewMode === ViewMode.notDone
        ? tasks.filter((task) => !task.is_done)
        : tasks
  );
  let inputElement: HTMLInputElement;
  function handlechangeViewMode(newviewmode: number) {
    viewMode = newviewmode;
  }
  // Handle task change
  async function handleTaskChange(task: task) {
    console.log("task change:", task);
    try {
      await updateTask($state.snapshot(task));
      await loadtask();
    } catch (error) {
      console.log("error:", error);
    }
  }

  function animate(node: HTMLElement, index: number) {
    SlideIn(node, index);
  }

  async function handleDelete(task: task) {
    try {
      await deletetask(task);
      await loadtask();
    } catch (error) {}
  }

  async function handleadd() {
    let value = inputElement.value;
    try {
      await addtask(value);
    } catch (error) {}
    inputElement.value = "";
    loadtask();
  }

  async function loadtask() {
    try {
      let Tasks = await getTasks();
      tasks = Tasks;
    } catch (err) {}
  }
  onMount(() => {
    loadtask();
  });
</script>

<main class=" min-h-screen p-4">
  <div class="flex justify-end" id="views button container">
    <div class="join join-vertical" id="views button">
      <button
        class="btn btn-primary {viewMode === ViewMode.All ? 'btn-outline' : ''}"
        onclick={() => {
          handlechangeViewMode(ViewMode.All);
        }}
      >
        all tasks
      </button>
      <button
        class="btn btn-secondary {viewMode === ViewMode.done
          ? 'btn-outline'
          : ''}"
        onclick={() => {
          handlechangeViewMode(ViewMode.done);
        }}
      >
        done tasks
      </button>
      <button
        class="btn btn-accent {viewMode === ViewMode.notDone
          ? 'btn-outline'
          : ''}"
        onclick={() => {
          handlechangeViewMode(ViewMode.notDone);
        }}
      >
        not done tasks
      </button>
    </div>
  </div>
  <div id="inner">
    <div class="grid grid-cols-3 grid-rows-3" id="task_list_container">
      <div class="overflow-y-auto col-span-2 col-start-2" id="task list">
        {#each viewTasks as task, index}
          <div use:animate={index} out:slideFade={{ index }}>
            <Task
              id={task.id}
              is_done={task.is_done}
              name={task.name}
              taskChange={handleTaskChange}
              ondelete={handleDelete}
            />
          </div>
        {/each}
      </div>

      <div class="size-32 bottom-0 mb-30 w-screen" id="input container">
        <fieldset
          class="fieldset bg-base-200 border border-base-300 p-4 rounded-box w-full flex flex-row justify-center text-center"
        >
          <legend class="fieldset-legend">new task</legend>
          <div class="join">
            <input
              type="text"
              name="taskname"
              id="takname"
              bind:this={inputElement}
              class="input join-item"
              placeholder="task name"
            />
            <button
              class="btn btn-outline btn-primary join-item"
              onclick={handleadd}>add</button
            >
          </div>
        </fieldset>
      </div>
    </div>
  </div>
</main>

<style>
  main {
    text-align: center;
    padding: 2rem;
    height: 100vh;
    position: relative;
    overflow: hidden;
  }
</style>
