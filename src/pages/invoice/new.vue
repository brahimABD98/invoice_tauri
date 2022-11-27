<script setup >
import { onMounted, ref } from 'vue'
import { token } from '@formkit/utils'
import { invoke } from "@tauri-apps/api/tauri";

const value = ref();
const invoicelines = ref([])
const invoiceline = ref()
const invoice = ref("");
const items = ref([token()])
const addItem = () => {
  items.value.push(token())
}
const removeItem = (item) => {
  items.value.pop(item)

}
async function newInvoice() {

  invoice.value = await invoke('new_invoice').catch(e => console.log(e));

}
onMounted(() => {
  newInvoice();
})


</script>

<template>
  <div>

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
              <input type="text" placeholder="#" v-model="invoice.taux" class="input input-bordered input-info w-full max-w-xs" />
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
                          <input type="text" placeholder="produit" class="input input-bordered w-auto border-emerald-50" />
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

    <pre wrap>{{invoice}}</pre>
  </div>
</template>
<style>

</style>