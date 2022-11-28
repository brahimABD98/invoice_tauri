<script setup >
import { onBeforeMount, onMounted, ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

const invoice = ref()
const resolvedinvoice=ref()
async function newInvoice() {
  await invoke('new_invoice').then((res) => {
    switch (typeof (res)) {
      case "string":
        return invoice.value = JSON.parse(res)
    }
  })
}

async function resolve() {
  let i = JSON.stringify(invoice.value)
  await invoke('resolve_invoice', { invoice: i }).then((res) => {
    console.log(res);
    switch (typeof (res)) {
      case "string":
        return resolvedinvoice.value = JSON.parse(res);
    }
  }).catch((error) => error);
}
onBeforeMount(() => {
  newInvoice();
})


</script>

<template>
  <div>
    <button @click="resolve()" class="btn btn-primary">call resolve</button>
    <p class="text-3xl">Ajouter une facture</p>
    <hr class="my-5" />
    <div class="mt-5">
      <form>
        <div class="flex flex-col">
          <div class="flex flex-row mx-3 ">
            <div class="form-control ">
              <label class="label">
                <span class="label-text text-xl ">N°</span>
              </label>
              <input type="number" placeholder="#" v-if="invoice" v-model="invoice.ttc"
                class="input input-bordered input-info w-full max-w-xs" />
            </div>
            <div class="form-control mx-3">
              <label class="label">
                <span class="label-text text-xl">Client</span>
              </label>
              <input type="text" placeholder="" class="input input-bordered input-info w-full max-w-xs" />
            </div>
            <div class="form-control mx-3">
              <label class="label">
                <span class="label-text text-xl">date</span>
              </label>
              <input type="date" placeholder="" class="input input-bordered input-info w-full max-w-xs" />
            </div>

          </div>
          <div class="mt-5">
            <div class="overflow-x-auto w-auto">
              <table class="table w-auto">
                <!-- head -->
                <thead>
                  <tr>
                    <!-- <th>
                      <label>
                        <input type="checkbox" class="checkbox" />
                      </label>
                    </th> -->
                    <th>Produit </th>
                    <th>Job</th>
                    <th>Favorite Color</th>
                    <th></th>
                    <th></th>
                  </tr>
                </thead>
                <tbody>
                  <!-- row 1 -->
                  <tr>
                    <!-- <th>
                      <label>
                        <input type="checkbox" class="checkbox" />
                      </label>
                    </th> -->
                    <td>
                      <div class="flex items-center space-x-3">

                        <div>
                          <input type="text" placeholder="produit"
                            class="input input-bordered w-auto border-emerald-50" />
                        </div>
                      </div>
                    </td>
                    <td>
                      Zemlak, Daniel and Leannon
                      <br />
                      <span class="badge badge-ghost badge-sm">Desktop Support Technician</span>
                    </td>
                    <td>Purple</td>
                    <th>
                      <button class="btn btn-ghost btn-xs">details</button>
                    </th>
                  </tr>

                </tbody>
                <!-- foot -->


              </table>
            </div>
          </div>
        </div>

      </form>
    </div>

    <pre wrap>{{ resolvedinvoice }}</pre>
  </div>
</template>
<style>

</style>