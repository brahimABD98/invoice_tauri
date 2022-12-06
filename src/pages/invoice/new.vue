<script setup>
import { onBeforeMount, onMounted, ref } from "vue";
import { token } from "@formkit/utils";
import { invoke } from "@tauri-apps/api/tauri";

const invoice = ref();

const addItem = async () => {
  const newinvoiceline = await newInvoiceLine().catch((e) => "error" + error);
  invoice.value.invoicelinelist.push(newinvoiceline);
};

const removeItem = (item) => {
  invoice.value.invoicelinelist.pop(item);
};

async function newInvoiceLine() {
  const res = await invoke("new_invoice_line").catch(
    (error) => "error" + error
  );
  return JSON.parse(res);
}

async function newInvoice() {
  invoice.value = await invoke("new_invoice")
    .then((str) => {
      return JSON.parse(str);
    })
    .catch((e) => "error" + error);
}
onBeforeMount(() => {
  newInvoice();
});
</script>

<template>
  <div>
    <FormKit
      type="form"
      name="Invoice"
      id="registration-example"
      submit-label="Register"
      v-if="invoice"
      #default="{ value }"
    >
      <div class="flex flex-col">
        <h1>Facutre!</h1>

        <p>Remplissez les champs selon vos besoins</p>
        <hr />
        <div class="flex flex-row my-5 items-center">
          <FormKit
            type="number"
            name="number"
            v-model="invoice.number"
            input-class="w-32 input input-bordered input-info"
            label="Facutre numero"
            min="1"
          />
          <div class="mx-3"></div>
          <FormKit
            type="text"
            name="client"
            v-model="invoice.client"
            input-class="w-32 input input-bordered input-info"
            label="tapez le nom de votre client"
            placeholder="Jane Doe"
            help=""
            validation="required"
          />
          <div class="mx-3"></div>
          <FormKit
            type="date"
            name="date"
            v-model="invoice.date"
            input-class="w-38  input input-bordered input-info"
            label="date"
            placeholder=""
            help=""
            validation="required"
          />
        </div>

        <div class="overflow-x-auto w-full">
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
            <tbody>
              <FormKit name="invoicelinelist" type="list">
                <FormKit
                  type="group"
                  name="invoiceline"
                  v-if="invoice"
                  v-for="invoiceline in invoice.invoicelinelist"
                >
                  <tr>
                    <th>
                      <label>
                        <input type="checkbox" class="checkbox" />
                      </label>
                    </th>
                    <td>
                      <div class="flex items-center">
                        <button
                          class="btn btn-outline btn-error"
                          @click.prevent="removeItem(invoiceline)"
                        >
                          supprimer
                        </button>
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center">
                        <FormKit
                          type="text"
                          name="produit"
                          v-model="invoiceline.produit"
                          placeholder="produit"
                          input-class="w-42 input input-bordered input-info"
                          validation="required"
                        />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center">
                        <FormKit
                          type="number"
                          name="qte"
                          v-model="invoiceline.qte"
                          validation="required"
                          input-class="w-24 input input-bordered input-info"
                          min="1"
                        />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center">
                        <FormKit
                          type="number"
                          name="taux"
                          v-model="invoiceline.taux"
                          placeholder="taux tva"
                          input-class="w-24 input input-bordered input-info"
                          validation="required"
                        />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center">
                        <FormKit
                          type="text"
                          v-model="invoiceline.puht"
                          name="htva"
                          input-class="w-32 input input-bordered input-info"
                          :validation="[
                            [
                              'matches',
                              /^(?:[0-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/,
                            ],
                          ]"
                        />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center">
                        <FormKit
                          type="text"
                          name="tva"
                          v-model="invoiceline.tva"
                          input-class="w-28 input input-bordered input-info"
                          :validation="[
                            [
                              'matches',
                              /^(?:[0-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/,
                            ],
                          ]"
                        />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center">
                        <FormKit
                          type="text"
                          name="ttc"
                          v-model="invoiceline.ttc"
                          input-class="w-28 input input-bordered input-info"
                          :validation="[
                            [
                              'matches',
                              /^(?:[0-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/,
                            ],
                          ]"
                        />
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
                  <button type="button" @click.prevent="addItem">
                    + Add Email
                  </button>
                </td>
              </tr>
            </tfoot>
            <!-- end foot -->
          </table>
        </div>
        <div class="items-end mt-5">
          <FormKit
            v-model="invoice.htva"
            type="text"
            name="htva"
            label="total hors tva"
            :validation="[
              ['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/],
            ]"
            input-class="w-32 input input-bordered input-info"
          />
          <FormKit
            type="text"
            name="timbre"
            label="timbre"
            v-model="invoice.timbre"
            :validation="[
              ['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/],
            ]"
            input-class="w-32 input input-bordered input-info"
          />
          <FormKit
            type="number"
            name="taux"
            label="taux tva global"
            v-model="invoice.taux"
            validation="required"
            min="1"
            max="100"
            input-class="w-32 input input-bordered input-info"
          />
        </div>
        <div class="divider"></div>

        <!-- <pre wrap>{{ value }}</pre> -->
        <pre wrap>{{ invoice }}</pre>
        <div class="divider"></div>
      </div>
    </FormKit>
  </div>
</template>
<style></style>
