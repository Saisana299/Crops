<script lang="ts">
  import List, { Item, Text, Graphic } from "@smui/list";
  import Button from "@smui/button";
  import { invoke } from "@tauri-apps/api/tauri";

  let messages = ["test1", "test2"];

  const handleClickUpdate = async () => {
    messages = await invoke<string[]>("fetch_messages");
  };
</script>

<main>
  <Button on:click={handleClickUpdate}>Update</Button>
  <List>
    {#each messages as message}
      <Item
        ><Graphic class="material-icons">send</Graphic><Text>{message}</Text
        ></Item>
    {/each}
  </List>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
  }

  img {
    height: 16rem;
    width: 16rem;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4rem;
    font-weight: 100;
    line-height: 1.1;
    margin: 2rem auto;
    max-width: 14rem;
  }

  p {
    max-width: 14rem;
    margin: 1rem auto;
    line-height: 1.35;
  }

  @media (min-width: 480px) {
    h1 {
      max-width: none;
    }

    p {
      max-width: none;
    }
  }
</style>
