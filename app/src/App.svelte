<script>
  // @ts-nocheck

  import logo from "./assets/demo.png";
  import { createDbWorker } from "sql.js-httpvfs";
  import { onMount } from "svelte";

  let initial = [];
  let aulas = 0;

  onMount(async () => {
    console.log("pila");
    const worker = await createDbWorker(
      [
        {
          from: "inline",
          config: {
            serverMode: "full",
            requestChunkSize: 4096,
            url: "falta.db",
          },
        },
      ],
      "/sqlite.worker.js",
      "/sql-wasm.wasm",
      10 * 1024 * 1024 // optional, defaults to Infinity
    );

    const result = await worker.db.exec(
      `SELECT name, party, COUNT(*) FROM registry WHERE presence == 0 GROUP BY name, party ORDER BY COUNT(*) DESC;`
    );

    initial = result[0].values;
    console.log(initial);

    const result_aulas = await worker.db.exec(
      `SELECT COUNT(DISTINCT date) FROM registry;`
    );

    console.log(result_aulas);
    aulas = result_aulas[0].values[0];
  });

  let caderneta = () => {
    alert("demo");
  };

  let aluno = "";
</script>

<main>
  <center>
    <img
      src={logo}
      alt="presidente da assembleia da republica com um livro de presenças"
    />
  </center>

  <h2>Aulas registadas: {aulas}</h2>
  <!--div style="display: flex;">
    <select name="cars" id="cars">
      <option value="volvo">Todas as bancadas</option>
      <option value="saab">Saab</option>
      <option value="mercedes">Mercedes</option>
      <option value="audi">Audi</option>
    </select>

    <select name="cars" id="cars">
      <option value="volvo">Todas as aulinhas</option>
      <option value="saab">Saab</option>
      <option value="mercedes">Mercedes</option>
      <option value="audi">Audi</option>
    </select>
  </div-->

  <blockquote>
    Faltas escessivas segundo a <a
      href="https://www.pgdlisboa.pt/leis/lei_mostra_articulado.php?artigo_id=1793A0018&nid=1793&tabela=leis&pagina=1&ficha=1&so_miolo=&nversao="
      >Lei n.º 51/2012 Artigo 18 1º Ponto Alínea a</a
    > aparecem a vermelho!
  </blockquote>

  <table>
    <thead>
      <tr>
        <th>Deputado</th>
        <th style="text-align: center;">Bancada</th>
        <th style="text-align: right;">Faltas</th>
      </tr>
    </thead>
    <tbody>
      {#each initial as arr, i}
        <tr>
          {#if arr[2] > 10}
            <td style="color: red;">{arr[0]}</td>
          {:else}
            <td>{arr[0]}</td>
          {/if}
          <td style="text-align: center;">{arr[1]}</td>
          <td style="text-align: right;">
            {#if arr[2] > 10}
              <!-- svelte-ignore a11y-missing-attribute -->
              <a
                style="cursor: pointer"
                on:click={() => {
                  caderneta();
                }}>{arr[2]}</a
              >
            {:else}
              {arr[2]}
            {/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>

  <br />
  <hr />
  <p>
    Feito com ❤ por <a href="https:luissilva.eu">Luís Silva</a> podes consultar
    o código-fonte da página <a href="#todo">aqui</a>.
  </p>

  <p class="read-the-docs">
    Obviamente que isto pode ter erros e como tal, não levem isto muito a peito.
    Mandem mail para resolver 💌
  </p>
</main>

<style>
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }
</style>
