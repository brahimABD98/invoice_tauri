<script setup>
import { ref } from 'vue'
const submitted = ref(false)
const submitHandler = async () => {
  // Let's pretend this is an ajax request:
  await new Promise((r) => setTimeout(r, 1000))
  submitted.value = true
}
</script>

<template>
  <div>
    <FormKit type="form" name="Invoice" id="registration-example" :form-class="submitted ? 'hide' : 'show'"
      submit-label="Register" @submit="submitHandler" :actions="false" #default="{ value }">
      <h1>Facutre!</h1>
      <p>
        Remplissez les champs selon vos besoins
      </p>
      <hr />
      <FormKit type="text" name="Client" label="tapez le nom de votre client" placeholder="Jane Doe" help=""
        validation="required" />
      <FormKit type="date" name="date" label="date" placeholder="" help="" validation="required" />
      <FormKit type="number" name="number" label="Facutre numero" min="1" />

      <FormKit type="text" name="htva" label="total hors tva"
        :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
      <FormKit type="text" name="timbre" label="timbre"
        :validation="[['matches', /^(?:[1-9]\d*|0(?!(?:\.0+)?$))?(?:\.\d+)?$/]]" />
      <FormKit type="number" name="taux" label="taux tva global" validation="required" min="1" max="100" />
      <!-- <FormKit id="repeater" name="invoiceline" type="repeater" label="invoiceline">
        <FormKit type="text" label="produit" name="produit" validation="required" />
      </FormKit> -->

   
      <FormKit type="submit" label="Register" />
      <pre wrap>{{ value }}</pre>
    </FormKit>
    <div v-if="submitted">
      <h2>Submission successful!</h2>
    </div>
  </div>
</template>