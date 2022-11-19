<script setup >
import { ref } from 'vue'
import { token } from '@formkit/utils'

// const submitted = ref(false)

const value = ref();
const invoicelines = ref([])
const invoiceline = ref()
const items = ref([token()])
// console.log(value.value)
const addItem = () => {
  items.value.push(token())
}





</script>

<template>
  <div>
    <FormKit type="form" name="Invoice" id="registration-example" submit-label="Register" #default="{ value }">
      <h1>Facutre!</h1>
      <p>
        Remplissez les champs selon vos besoins
      </p>
      <hr />

      <div class="flex flex-row my-5 items-center  ">

        <FormKit type="number" name="number" label="Facutre numero" min="1" />
        <div class="mx-3"></div>
        <FormKit type="text" name="Client" label="tapez le nom de votre client" placeholder="Jane Doe" help=""
          validation="required" />
          <div class="mx-3"></div>
        <FormKit type="date" name="date" label="date" placeholder="" help="" validation="required" />

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
              <th>Produit</th>
              <th>quantité</th>
              <th>taux tva</th>
              <th>prix unitaire hors tva</th>
              <th>tva</th>
              <th>ttc</th>
            </tr>
          </thead>
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
                    <div class="flex items-center space-x-3">
                      <FormKit type="text" name="produit" placeholder="produit" validation="required" />
                    </div>
                  </td>
                  <td>
                    <div class="flex items-center space-x-3">
                      <FormKit type="number" name="qte" validation="required" min="1" value="1" />
                    </div>
                  </td>
                  <td>
                    <div class="flex items-center space-x-3">
                      <FormKit type="number" name="taux" placeholder="taux tva" validation="required" value="20" />
                    </div>
                  </td>
                  <td>
                    <div class="flex items-center space-x-3">
                      <FormKit type="text" name="htva"
                        :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
                    </div>
                  </td>
                  <td>
                    <div class="flex items-center space-x-3">
                      <FormKit type="text" name="tva"
                        :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
                    </div>
                  </td>
                  <td>
                    <div class="flex items-center space-x-3">
                      <FormKit type="text" name="ttc"
                        :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
                    </div>
                  </td>

               
            </tr>
          </FormKit>
              </FormKit>
            <!-- row 1 -->
            <!-- <tr>
        <th>
          <label>
            <input type="checkbox" class="checkbox" />
          </label>
        </th>
        <td>
          <div class="flex items-center space-x-3">
            <div class="avatar">
              <div class="mask mask-squircle w-12 h-12">
                <img src="/tailwind-css-component-profile-2@56w.png" alt="Avatar Tailwind CSS Component" />
              </div>
            </div>
            <div>
              <div class="font-bold">Hart Hagerty</div>
              <div class="text-sm opacity-50">United States</div>
            </div>
          </div>
        </td>
        <td>
          Zemlak, Daniel and Leannon
          <br/>
          <span class="badge badge-ghost badge-sm">Desktop Support Technician</span>
        </td>
        <td>Purple</td>
        <th>
          <button class="btn btn-ghost btn-xs">details</button>
        </th>
      </tr> -->

          </tbody>
          <!-- foot -->
          <tfoot>
            <tr>
              <td>
                <button type="button" @click.prevent="addItem">+ Add Email</button>

              </td>
            </tr>
          </tfoot>

        </table>
      </div>

      <!-- <FormKit name="invoicelines" v-model="invoicelines" type="list">
        <FormKit type="group" name="invoiceline" v-model="invoiceline" v-for="item in items ">
          <div class="flex flex-row">
            <FormKit type="text" name="produit" label="Produit" validation="required" />
            <FormKit type="number" name="qte" label="quantite" validation="required" min="1" value="1" />
            <FormKit type="text" name="htva" label="prix untaire hors tva"
              :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
            <FormKit type="text" name="tva" label="tva"
              :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
            <FormKit type="text" name="ttc" label="ttc"
              :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
            <FormKit type="number" name="taux" label="taux tva" validation="required" value="20" />
          </div>
        </FormKit>

        <div class="divider"></div>
        <button type="button" @click.prevent="addItem">+ Add Email</button>
        <div class="divider"></div>

      </FormKit> -->

      <div class="">

        <FormKit type="text" name="htva" label="total hors tva"
          :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
        <FormKit type="text" name="timbre" label="timbre"
          :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
        <FormKit type="number" name="taux" label="taux tva global" validation="required" min="1" max="100" value="20" />
      </div>

      <div class="divider"></div>






      <FormKit type="submit" label="Register" />
      <pre wrap>{{ value }}</pre>
    </FormKit>

  </div>
</template>