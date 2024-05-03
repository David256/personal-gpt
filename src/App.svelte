<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { emit } from "@tauri-apps/api/event";

  import Microphone from "./lib/Microphone.svelte";
  import DeviceList from "./lib/DeviceList.svelte";
  import Modal from "./lib/Modal.svelte";
  import AI from "./lib/AI.svelte";

  let deviceNames: string[] = [];
  let deviceIndex: number | null = null;

  let wasRecordingError = false;

  let modalMessage = "";
  let isOpenModal = false;
  const openModal = () => (isOpenModal = true);
  const closeModal = () => (isOpenModal = false);

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

    await emit("new-audio", { filename: result });
  };

  const requestDeviceNames = async () => {
    const result = await invoke("list_devices");
    console.log("devices:", result);
    return result as string[];
  };

  const getErrorMessage = async () => {
    const result = await invoke("get_error_message");
    // console.log("error message:", result);
    return result as boolean;
  };

  const updateDeviceIndex = (index: number) => {
    console.log("update device index to:", index);
    deviceIndex = index;
  };

  requestDeviceNames().then((names) => {
    console.log("requested devices:", names);
    deviceNames = names;
  });

  setInterval(() => {
    getErrorMessage().then((message) => {
      if (message !== wasRecordingError) {
        wasRecordingError = message;
        modalMessage =
          "Error during the recording: Cannot read from selected devices";
        openModal();
      }
    });
  }, 1000);
</script>

<main class="container app-container">
  <div class="row">
    <div class="col center">
      <h1 class="title">Personal GPT</h1>
    </div>
  </div>

  <div class="row h-full">
    <div class="col p-2 space-between">
      <Microphone {onStartRecording} {onStopRecording} />
      <AI />
      <DeviceList
        {deviceIndex}
        devices={deviceNames}
        onSelect={updateDeviceIndex}
      />
    </div>
  </div>
  <Modal title="Error" open={isOpenModal} onClose={closeModal}
    >{modalMessage}</Modal
  >
</main>

<style>
  .title {
    color: rgb(240, 252, 255);
  }
  .app-container {
    position: relative;
  }
</style>
