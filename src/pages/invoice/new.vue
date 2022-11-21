<script setup >
import { ref } from 'vue'
import { token } from '@formkit/utils'

const value = ref();
const invoicelines = ref([])
const invoiceline = ref()
const items = ref([token()])
// console.log(value.value)
const addItem = () => {
  items.value.push(token())
}
const removeItem = (item) => {
  items.value.pop(item)

}

</script>

<template>
  <div>
    <FormKit type="form" name="Invoice" id="registration-example" submit-label="Register" #default="{ value }">
      <div class="flex flex-col">
        <h1>Facutre!</h1>
        <p>
          Remplissez les champs selon vos besoins
        </p>
        <hr />

        <div class="flex flex-row my-5 items-center   ">

          <FormKit type="number" name="number" input-class="w-32 input input-bordered input-info" label="Facutre numero"
            min="1" />
          <div class="mx-3"></div>
          <FormKit type="text" name="Client" input-class="w-32 input input-bordered input-info"
            label="tapez le nom de votre client" placeholder="Jane Doe" help="" validation="required" />
          <div class="mx-3"></div>
          <FormKit type="date" name="date" input-class="w-38  input input-bordered input-info" label="date"
            placeholder="" help="" validation="required" />

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
            <tbody>
              <FormKit name="invoicelines" v-model="invoicelines" type="list">
                <FormKit type="group" name="invoiceline" v-model="invoiceline" v-for="item in items ">
                  <tr>
                    <th>
                      <label>
                        <input type="checkbox" class="checkbox" />
                      </label>
                    </th>
                    <td>
                      <div class="flex items-center">
                        <button class="btn btn-outline btn-error" @click="removeItem(item)">supprimer</button>
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center ">
                        <FormKit type="text" name="produit" placeholder="produit"
                          input-class="w-42 input input-bordered input-info" validation="required" />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center ">
                        <FormKit type="number" name="qte" validation="required"
                          input-class="w-24 input input-bordered input-info" min="1" value="1" />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center">
                        <FormKit type="number" name="taux" placeholder="taux tva"
                          input-class="w-24 input input-bordered input-info" validation="required" value="20" />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center ">
                        <FormKit type="text" name="htva" input-class="w-32 input input-bordered input-info"
                          :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center ">
                        <FormKit type="text" name="tva" input-class="w-28 input input-bordered input-info"
                          :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center ">
                        <FormKit type="text" name="ttc" input-class="w-28 input input-bordered input-info"
                          :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
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
          <FormKit type="text" name="htva" label="total hors tva"
            :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]"
            input-class="w-32 input input-bordered input-info" />
          <FormKit type="text" name="timbre" label="timbre"
            :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]"
            input-class="w-32 input input-bordered input-info" />
          <FormKit type="number" name="taux" label="taux tva global" validation="required" min="1" max="100" value="20"
            input-class="w-32 input input-bordered input-info" />
        </div>
        <div class="divider"></div>
        <!-- <FormKit type="submit" label="Register" /> -->
        <pre wrap>{{ value }}</pre>
      </div>
    </FormKit>

  </div>
</template>
<style>

</style>