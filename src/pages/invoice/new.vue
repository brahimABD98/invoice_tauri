<script setup >
import { computed, onBeforeMount, ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

const invoice = ref();

const invoicelines = computed(() => {
  return invoice.value.invoicelinelist;
})
async function reso() {

  const str = JSON.stringify(invoice.value);
  console.log("str" + str);
  await invoke('resolve_invoice', { invoice: str }).then((res) => {

    invoice.value = JSON.parse(res)
  }).catch((error) => "error:" + error)

}

const castNumber = (node) => {
  node.hook.input((value, next) => next(Number(value)))
}

const addItem = async () => {
  const newinvoiceline = await newInvoiceLine().catch((error) => 'error:' + error)
  invoice.value.invoicelinelist.push(newinvoiceline)
}

const removeItem = (item) => {
  invoice.value.invoicelinelist.pop(item)
}

async function newInvoiceLine() {
  const res = await invoke('new_invoice_line').catch((error) => 'error:' + error)
  return JSON.parse(res)
}

async function newInvoice() {
  invoice.value = await invoke('new_invoice').then(
    (str) => {
      return JSON.parse(str)
    }).catch(error => 'error:' + error);
}



onBeforeMount(() => {
  newInvoice();
})
</script>


<template>
  <div>

    <FormKit type="form" name="Invoice" id="registration-example" submit-label="Register" v-if="invoice">
      <div class="flex flex-col">
        <h1>Facutre!</h1>
        <button class="btn btn-square" @click.prevent="reso()">testouga</button>
        <p>
          Remplissez les champs selon vos besoins
        </p>
        <hr />
        <div class="flex flex-row my-5 items-center">

          <FormKit type="number" name="number" v-model="invoice.number" :plugins="[castNumber]"
            input-class="w-32 input input-bordered input-info" label="Facutre numero" min="1" />
          <div class="mx-3"></div>
          <FormKit type="text" name="client" v-model="invoice.client" input-class="w-32 input input-bordered input-info"
            label="tapez le nom de votre client" placeholder="Jane Doe" help="" validation="required" />
          <div class="mx-3"></div>
          <FormKit type="date" name="date" v-model="invoice.date" input-class="w-38  input input-bordered input-info"
            label="date" placeholder="" help="" validation="required" />

        </div>

        <div class="overflow-x-auto w-full ">
          <table class="table w-full">
            <!-- head -->
            <thead>
              <tr>
                <th>
                  <label>
                    <input type="checkbox" class="checkbox" />
                  </label>
                </th>
                <th></th>
                <th>Produit</th>
                <th>quantité</th>
                <th>taux tva</th>
                <th>prix unitaire hors tva</th>
                <th>tva</th>
                <th>ttc</th>
              </tr>
            </thead>
            <!-- body -->
            <tbody v-if="invoice">
              <FormKit name="invoicelinelist" type="list" v-if="invoicelines">
                <FormKit type="group" name="invoiceline" v-for="invoiceline in invoicelines ">

                  <tr>
                    <th>
                      <label>
                        <input type="checkbox" class="checkbox" />
                      </label>
                    </th>
                    <td>
                      <div class="flex items-center">
                        <button class="btn btn-outline btn-error"
                          @click.prevent="removeItem(invoiceline)">supprimer</button>
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center ">
                        <FormKit type="text" name="produit" v-model="invoiceline.produit" placeholder="produit"
                          input-class="w-42 input input-bordered input-info" validation="required" />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center ">
                        <FormKit type="number" name="qte" v-model="invoiceline.qte" validation="required"
                          :plugins="[castNumber]" input-class="w-24 input input-bordered input-info" min="1" />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center">
                        <FormKit type="number" name="taux" v-model="invoiceline.taux" placeholder="taux tva"
                          :plugins="[castNumber]" input-class="w-24 input input-bordered input-info"
                          validation="required" />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center ">
                        <FormKit type="text" v-model="invoiceline.puht" name="htva" :plugins="[castNumber]"
                          input-class="w-32 input input-bordered input-info"
                          :validation="[['matches', /^(?:[0-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center ">
                        <FormKit type="text" name="tva" v-model="invoiceline.tva" :plugins="[castNumber]"
                          input-class="w-28 input input-bordered input-info"
                          :validation="[['matches', /^(?:[0-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center ">
                        <FormKit type="text" name="ttc" v-model="invoiceline.ttc" :plugins="[castNumber]"
                          input-class="w-28 input input-bordered input-info"
                          :validation="[['matches', /^(?:[0-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
                      </div>
                    </td>
                  </tr>
                </FormKit>
              </FormKit>
            </tbody>
            <!-- end body -->
            <!-- foot -->
            <tfoot>
              <tr>
                <td>
                  <button type="button" @click.prevent="addItem">+ Add Email</button>
                </td>
              </tr>
            </tfoot>
            <!-- end foot -->
          </table>
        </div>
        <div class="items-end mt-5">
          <FormKit v-model="invoice.htva" type="text" name="htva" label="total hors tva" :plugins="[castNumber]"
            input-class="w-32 input input-bordered input-info" />
          <FormKit type="text" name="timbre" label="timbre" v-model="invoice.timbre" :plugins="[castNumber]"
            :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]"
            input-class="w-32 input input-bordered input-info" />
          <FormKit type="number" name="taux" label="taux tva global" v-model="invoice.taux" validation="required"
            :plugins="[castNumber]" min="1" max="100" input-class="w-32 input input-bordered input-info" />
        </div>
        <div class="divider"></div>

        <pre wrap>{{ invoice }}</pre>
        <div class="divider"></div>
      </div>
    </FormKit>
  </div>
</template>
<style>

</style>

