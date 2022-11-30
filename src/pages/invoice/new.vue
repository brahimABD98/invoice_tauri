<script setup >
import { onBeforeMount, onMounted, ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

const invoice = ref()
const resolvedinvoice = ref()
const empty_line = ref();
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

async function addnewInvoiceline() {
  await invoke('new_invoice_line').then((res) => {
    switch (typeof (res)) {
      case "string":
        invoice.value.invoicelinelist.push(res)

    }
  }).catch((e)=>console.log(e))
}
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
              <input type="number" placeholder="#" v-if="invoice" v-model="invoice.number"
                class="input input-bordered input-info w-full max-w-xs" />
            </div>
            <div class="form-control mx-3">
              <label class="label">
                <span class="label-text text-xl">Client</span>
              </label>
              <input type="text" placeholder="" v-if="invoice" v-model="invoice.client"
                class="input input-bordered input-info w-full max-w-xs" />
            </div>
            <div class="form-control mx-3">
              <label class="label">
                <span class="label-text text-xl">date</span>
              </label>
              <input type="date" placeholder="" v-if="invoice" v-model="invoice.date"
                class="input input-bordered input-info w-full max-w-xs" />
            </div>

          </div>
          <div class="mt-5">
            <div class="overflow-x-auto w-auto">
              <table class="table w-auto">
                <!-- head -->
                <thead>
                  <tr>
                    <th>
                      <label>
                        <input type="checkbox" class="checkbox" />
                      </label>
                    </th>
                    <th>Produit </th>
                    <th>quantite</th>
                    <th>taux</th>
                    <th>prix hors tva</th>
                    <th>TVA</th>
                    <th>TTC</th>
                  </tr>
                </thead>
                <tbody v-if="invoice" v-for="item in invoice.invoicelinelist">
                  <!-- row 1 -->
                  <tr>
                    <th>
                      <label>
                        <input type="checkbox" class="checkbox" />
                      </label>
                    </th>
                    <td>
                      <div class="flex items-center space-x-3">
                        <div>
                          <input type="text" placeholder="produit" v-if="invoice" v-model="item.produit"
                            class="input input-bordered w-38 border-emerald-50" />
                        </div>
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center space-x-3">
                        <div>
                          <input type="number" v-if="invoice" v-model="item.qte" placeholder="quantite"
                            class="input input-bordered w-24 border-emerald-50" />
                        </div>
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center space-x-3">
                        <div>
                          <input type="number" v-if="invoice" v-model="item.taux" placeholder="taux"
                            class="input input-bordered w-24 border-emerald-50" />
                        </div>
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center space-x-3">
                        <div>
                          <input type="number" v-if="invoice" v-model="item.htva" placeholder="htva"
                            class="input input-bordered w-38 border-emerald-50" />
                        </div>
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center space-x-3">
                        <div>
                          <input type="number" v-if="invoice" v-model="item.tva" placeholder="tva"
                            class="input input-bordered w-38 border-emerald-50" />
                        </div>
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center space-x-3">
                        <div>
                          <input type="number" v-if="invoice" v-model="item.ttc" placeholder="ttc"
                            class="input input-bordered  w-24 border-emerald-50" />
                        </div>
                      </div>
                    </td>
                  </tr>

                </tbody>
                <!-- foot -->
                <tfoot>
                  <tr>
                    <td>
                      <button type="button" v-if="invoice" @click.prevent="addnewInvoiceline()">+ Add
                        Email</button>
                    </td>
                  </tr>
                </tfoot>

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