<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import Microphone from "./lib/Microphone.svelte";
  import DeviceList from "./lib/DeviceList.svelte";

  let deviceNames: string[] = [];
  let deviceIndex = 0;

  let isRecording = false;

  const onStartRecording = async () => {
    if (isRecording) return;
    const result = await invoke("start_recording", { index: deviceIndex });
    console.log("start recording:", result);
    isRecording = true;
  };

  const onStopRecording = async () => {
    if (!isRecording) return;
    const result = await invoke("stop_recording");
    console.log("stop recording:", result);
    isRecording = false;
  };

  const requestDeviceNames = async () => {
    const result = await invoke("list_devices");
    console.log("devices:", result);
    return result as string[];
  };

  const updateDeviceIndex = (index: number) => {
    deviceIndex = index;
  };

  requestDeviceNames().then((names) => {
    console.log("devices:", names);
    deviceNames = names;
  });
</script>

<main class="container">
  <div class="row">
    <div class="col center">
      <h1 class="title">Personal GPT</h1>
    </div>
  </div>

  <div class="row h-full">
    <div class="col p-2 space-between">
      <Microphone {onStartRecording} {onStopRecording} />
      <DeviceList
        {deviceIndex}
        devices={deviceNames}
        onSelect={updateDeviceIndex}
      />
    </div>
  </div>
</main>

<style>
  .title {
    color: rgb(240, 252, 255);
  }
</style>
