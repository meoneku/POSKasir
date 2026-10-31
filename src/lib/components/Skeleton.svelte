<script lang="ts">
  interface Props {
    type?: "card" | "table" | "grid" | "line";
    count?: number;
    rows?: number;
    cols?: number;
  }

  let {
    type = "card",
    count = 4,
    rows = 5,
    cols = 4,
  }: Props = $props();
</script>

{#if type === "card"}
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
    {#each Array(count) as _}
      <div class="bg-white border border-slate-200 rounded-lg p-5 shadow-xs animate-pulse">
        <div class="h-3 w-1/3 bg-slate-200 rounded mb-3"></div>
        <div class="h-7 w-2/3 bg-slate-300 rounded mb-2"></div>
        <div class="h-3 w-1/2 bg-slate-200 rounded"></div>
      </div>
    {/each}
  </div>
{:else if type === "grid"}
  <div class="grid grid-cols-2 sm:grid-cols-3 xl:grid-cols-4 gap-3 p-3">
    {#each Array(count) as _}
      <div class="bg-white border border-slate-200 rounded-lg p-3 animate-pulse">
        <div class="h-28 bg-slate-200 rounded-md mb-2.5"></div>
        <div class="h-4 w-3/4 bg-slate-300 rounded mb-1.5"></div>
        <div class="h-3 w-1/3 bg-slate-200 rounded mb-2"></div>
        <div class="flex justify-between items-center pt-1 border-t border-slate-100">
          <div class="h-4 w-2/5 bg-slate-300 rounded"></div>
          <div class="h-7 w-16 bg-slate-200 rounded"></div>
        </div>
      </div>
    {/each}
  </div>
{:else if type === "table"}
  <div class="bg-white border border-slate-200 rounded-lg overflow-hidden animate-pulse">
    <div class="bg-slate-100 border-b border-slate-200 h-10 px-4 flex items-center gap-4">
      {#each Array(cols) as _}
        <div class="h-3.5 bg-slate-300 rounded flex-1"></div>
      {/each}
    </div>
    <div class="divide-y divide-slate-100">
      {#each Array(rows) as _}
        <div class="px-4 py-3.5 flex items-center gap-4">
          {#each Array(cols) as _}
            <div class="h-3.5 bg-slate-200 rounded flex-1"></div>
          {/each}
        </div>
      {/each}
    </div>
  </div>
{:else}
  <div class="space-y-2 animate-pulse">
    {#each Array(count) as _}
      <div class="h-4 bg-slate-200 rounded w-full"></div>
    {/each}
  </div>
{/if}
