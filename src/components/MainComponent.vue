<template>
  <div class="column container items-center text-center q-pa-xl q-gutter-lg">
    <div class="col row justify-center"><q-icon name="sym_o_replace_image" size="6em" /></div>

    <div class="col"><p>Drag and drop images or click to browse. Supported formats: {{ supportedInputFormats }}</p></div>
    <div class="col-auto row items-center justify-center q-gutter-sm q-mx-md">
      <p class="col-auto">Output format: </p>
      <q-select class="col output-selector q-mb-md" rounded outlined v-model="outputFormat" :options="supportedOutputFormats" label="Format" />
    </div>
    <div class="col-auto row justify-center">
      <div class="column items-center q-pa-lg drag-n-drop">
        <q-icon name="sym_o_upload" size="6em" />
        <p>Drag and drop images here</p>
        <p>or</p>
        <q-file outlined v-model="inputFiles" label="Click to browse" class="q-mb-md">
          <template v-slot:prepend>
            <q-icon name="sym_o_attach_file" />
          </template>
        </q-file>
      </div>
    </div>
    <div class="col">
      <q-btn :disabled="!inputFiles || !outputFormat" color="primary" label="Convert" @click="convert" />
    </div>

  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { convertHeic } from 'src/boot/axios';
import { AxiosError } from 'axios';

export default defineComponent({
  name: 'MainComponent',
  setup() {
    const supportedInputFormats = 'HEIC, HEIF, JPEG, PNG, TIFF, BMP, GIF'; // TODO
    const supportedOutputFormats = ['JPEG', 'PNG', 'TIFF', 'BMP', 'GIF']; // TODO

    const outputFormat = ref<string|null>(null);
    const inputFiles = ref<File | File[] | null>(null);

    const convert = async () => {
      console.log('Converting', inputFiles.value, 'to', outputFormat.value);
      try {
        const result = await convertHeic("./images", "png", true);
        console.log(result.data);
      } catch (e) {
        console.log("Error: " + String(e) + (e instanceof AxiosError ? " - " + JSON.stringify(e.response?.data) : ''));
      }
    };
    return {
      supportedInputFormats,
      supportedOutputFormats,
      outputFormat,
      inputFiles,
      convert
    };
  },
});

</script>

<style lang="scss" scoped>
.output-selector {
  max-width: 125px;
}

.output-selector::v-deep(.q-field__control) {
  border-radius: 1em !important;
}

.container {
  display:grid;
  row-gap: 1em;
  height: auto;
  width: 80%;
  color: var(--text-color);
}

.drag-n-drop {
  border: 2px dashed var(--q-color-primary);
  border-radius: 1em;
  border-style: dashed;
  width: 100%;
  height: 100%;
}
</style>
