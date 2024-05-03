<script lang="ts">
  export let deviceIndex;
  export let devices: string[] = [];
  export let onSelect: (index: number) => void = () => {};

  let tab: HTMLDivElement;
  let content: HTMLDivElement;

  let open = false;

  const onTabClick = () => {
    content.classList.toggle("raiseMenu");
    tab.classList.toggle("turnTab");
    open = !open;
  };

  const handleSelect = (index: number) => {
    onSelect(index);
    onClose();
  };

  const onClose = () => {
    content.classList.remove("raiseMenu");
    tab.classList.remove("turnTab");
    open = false;
  };
</script>

<!-- on:blur={() => open && onTabClick()} -->
<div class="dl-container">
  <div
    role="button"
    tabindex="0"
    bind:this={tab}
    on:keyup={(e) => e.key === "Enter" && onTabClick()}
    on:click={onTabClick}
    class="dl-tab"
  ></div>
  <div role="listbox" bind:this={content} class="dl-content">
    {#if devices.length === 0}
      <div class="dl-no-device">No device found</div>
    {:else}
      {#each devices as device, i}
        <div
          role="button"
          class={`dl-device ${i === deviceIndex ? "dl-selected" : ""}`}
          on:click={() => handleSelect(i)}
          on:keyup={(e) => e.key === "Enter" && handleSelect(i)}
          tabindex={i + 1}
        >
          {device}
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .dl-container {
    position: relative;
  }
  .dl-tab {
    /* height: 1rem; */
    border-bottom: 1rem solid rgba(144, 81, 230, 0.3);
    border-left: 10.5rem solid transparent;
    border-right: 10.5rem solid transparent;
    cursor: pointer;
    animation-name: lookatme;
    animation-delay: 10s;
    animation-duration: 10s;
    animation-iteration-count: infinite;
  }
  .dl-tab:hover {
    filter: drop-shadow(0 0 1rem #1aaec8);
  }
  @keyframes lookatme {
    0% {
      border-bottom: 1rem solid rgba(144, 81, 230, 0.3);
    }
    30% {
      border-bottom: 1rem solid rgba(144, 81, 230, 1);
    }
    40% {
      border-bottom: 1rem solid rgba(144, 81, 230, 0.3);
    }
    100% {
      border-bottom: 1rem solid rgba(144, 81, 230, 1);
    }
  }
  .dl-content {
    position: absolute;
    left: 0;
    right: 0;
    /* bottom: 1rem; */
    display: none;
    /* display: flex; */
    flex-direction: column;

    transition: all 3s;
    list-style-type: none;
    padding-left: 0;
  }
  .dl-selected {
    background-color: rgba(144, 81, 230, 0.4) !important;
    font-weight: 600 !important;
    text-decoration: underline;
  }
  .dl-no-device {
    background-color: rgba(144, 81, 230, 0.2);
    padding: 0.3rem;
    color: gray;
    font-style: italic;
    text-align: center;
    user-select: none;
  }
  .dl-device {
    background-color: rgba(144, 81, 230, 0.1);
    padding: 0.3rem;
    cursor: pointer;
    overflow-y: hidden;
    white-space: nowrap;
  }
  .dl-device:hover {
    background-color: rgba(144, 81, 230, 0.2);
    font-weight: 700;
    overflow-y: scroll;
  }
  /* :global(.raiseMenu) {
    bottom: 1rem;
    display: flex !important;
  }
  :global(.turnTab) {
    transform: rotate(180deg);
  } */
</style>
