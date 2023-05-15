<script>
    import { onMount } from "svelte";
    import { neoFetch as nf } from "./utils/neoFetch";
    let counter = 0;
    let someInput = "";

    let result;

    onMount(async () => {
        result = await nf("/hello");
    });
</script>

<main>
    <h1>Hello everybody, {result?.message}!!!</h1>
    <button on:click={() => (counter += 1)}>{counter}</button>
    <input bind:value={someInput} />
    <p>
        Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn
        how to build Svelte apps.
    </p>
    <p>{someInput}</p>
</main>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    h1 {
        color: #ff3e00;
        /* text-transform: uppercase; */
        font-size: 4em;
        font-weight: 100;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>
