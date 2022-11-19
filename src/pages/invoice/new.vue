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

      <div class="flex flex-row  ">

        <FormKit type="number" name="number" label="Facutre numero" min="1" />
        <div class="mx-3"></div>
        <FormKit type="text" name="Client" label="tapez le nom de votre client" placeholder="Jane Doe" help=""
          validation="required" />
        <FormKit type="date" name="date" label="date" placeholder="" help="" validation="required" />

      </div>

      <FormKit name="invoicelines" v-model="invoicelines" type="list">
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

      </FormKit>

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