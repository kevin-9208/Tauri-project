<template>
  <div style="padding: 40px; font-family: sans-serif; text-align: center;">
    <h1>桌面录屏工具</h1>
    <p>状态：{{ status }}</p>
    <button @click="startRecording" :disabled="isRecording" style="margin: 10px; padding: 10px 20px; font-size: 16px;">
      开始录屏
    </button>
    <button @click="stopRecording" :disabled="!isRecording" style="margin: 10px; padding: 10px 20px; font-size: 16px;">
      停止录屏
    </button>
    <p v-if="savedPath">已保存到：{{ savedPath }}</p>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const isRecording = ref(false)
const status = ref('未开始')
const savedPath = ref('')

async function startRecording() {
  try {
    await invoke('start_recording')
    isRecording.value = true
    status.value = '录制中...'
    savedPath.value = ''
  } catch (e) {
    status.value = '启动失败: ' + e
  }
}

async function stopRecording() {
  try {
    const path = await invoke('stop_recording')
    isRecording.value = false
    status.value = '已停止'
    savedPath.value = path
  } catch (e) {
    status.value = '停止失败: ' + e
  }
}
</script>
