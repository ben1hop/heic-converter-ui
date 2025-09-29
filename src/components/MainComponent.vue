<template>
  <div class="column container items-center text-center q-pa-xl q-gutter-lg">
    <div class="col row justify-center"><q-icon name="sym_o_replace_image" size="6em" /></div>

    <div class="col">
      <p>
        Drag and drop images or click to browse. Supported formats:
        {{ Object.values(SupportedInputFormat) }}
      </p>
    </div>
    <div class="col-auto row items-center justify-center q-gutter-sm q-mx-md">
      <p class="col-auto">Output format:</p>
      <q-select
        class="col output-selector q-mb-md"
        rounded
        outlined
        v-model="convertArgs.outputFormat"
        :options="Object.values(SupportedOutputFormat)"
        label="Format"
      />
    </div>
    <div class="col-auto row justify-center">
      <div class="column items-center q-pa-lg drag-n-drop">
        <q-icon name="sym_o_upload" size="6em" />
        <p>Drag and drop images here</p>
        <p>or</p>
        <div class="row selector-rect q-pa-md items-center">
          <q-btn
            outlined
            v-model="convertArgs.inputFiles"
            icon="sym_o_attach_file"
            label="Select files"
            @click="selectFile"
            class="q-mb-md col-auto q-mt-md"
          />
          <div class="column items-center justify-center" style="width: 52px">
            <div class="col flex flex-center">
              <q-separator vertical style="height: 16px" color="primary" />
            </div>
            <div class="text-caption q-my-xs">or</div>
            <div class="col flex flex-center">
              <q-separator vertical style="height: 16px" color="primary" />
            </div>
          </div>
          <q-btn
            outlined
            v-model="convertArgs.inputFiles"
            icon="sym_r_folder"
            label="Select directory"
            @click="selectFolder"
            class="q-mb-md col-auto q-mt-md"
          />
        </div>
      </div>
    </div>
    <div class="col">
      <q-checkbox v-model="convertArgs.delete"></q-checkbox>
      <q-btn :disabled="missingArgs" color="primary" label="Convert" @click="convert" />
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { desktopDir } from '@tauri-apps/api/path';
import { SupportedInputFormat, SupportedOutputFormat } from './models';

interface ConvertArgs {
  inputFiles: string | string[] | null;
  outputFormat: SupportedOutputFormat | null;
  delete: boolean;
}

export default defineComponent({
  name: 'MainComponent',
  setup() {
    const convertArgs = ref<ConvertArgs>({ inputFiles: null, outputFormat: null, delete: false });
    const convert = async () => {
      console.log('Converting', convertArgs.value.inputFiles, 'to', convertArgs.value.outputFormat);
      try {
        const result = await invoke('convert', convertArgs.value);
        if (result) {
          console.log(result);
        }
      } catch (e) {
        console.log('Error: ' + String(e));
      }
    };

    const selectFile = async () => {
      try {
        const selected = await open({
          multiple: true,
          filters: [
            {
              name: 'Image',
              extensions: Object.values(SupportedInputFormat),
            },
          ],
        });
        if (Array.isArray(selected)) {
          // user selected multiple files
          console.log(selected);
        } else if (selected === null) {
          // user cancelled the selection
          console.log(selected);
        } else {
          // user selected a single file
          console.log(selected);
        }
      } catch (e) {
        console.log('Error: ' + String(e));
      }
    };

    const selectFolder = async () => {
      try {
        const selected = await open({
          multiple: false,
          directory: true,
          defaultPath: await desktopDir(),
        });
        if (selected === null) {
          // user cancelled the selection
          console.log(selected);
        } else {
          // user selected a single file
          console.log(selected);
        }
      } catch (e) {
        console.log('Error: ' + String(e));
      }
    };

    const missingArgs = computed(
      () => !convertArgs.value.inputFiles || convertArgs.value.outputFormat,
    );
    return {
      convertArgs,
      SupportedInputFormat,
      SupportedOutputFormat,
      convert,
      selectFile,
      selectFolder,
      missingArgs,
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
  display: grid;
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

.selector-rect {
  border-width: 1px;
  border-color: var(--q-color-primary);
  border-radius: 1em;
  border-style: solid;
}

.q-btn {
  width: 200px;
  background: var(--header-color-bg);
}
</style>
