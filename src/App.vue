<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";

const txtFilePath = ref("");
const excelTemplatePath = ref("");
const statusMsg = ref("");
const isProcessing = ref(false);

async function selectTxtFile() {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Text Files",
          extensions: ["txt"],
        },
      ],
    });

    if (selected && typeof selected === "string") {
      txtFilePath.value = selected;
      statusMsg.value = `已选择TXT文件: ${selected.split("/").pop()}`;
    } else if (selected === null) {
      // 用户取消了选择
      return;
    }
  } catch (error) {
    console.error("选择文件错误:", error);
    statusMsg.value = `选择文件失败: ${error.message || String(error)}`;
  }
}

async function selectExcelTemplate() {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Excel Files",
          extensions: ["xlsx", "xls"],
        },
      ],
    });

    if (selected && typeof selected === "string") {
      excelTemplatePath.value = selected;
      statusMsg.value = `已选择Excel模板: ${selected.split("/").pop()}`;
    } else if (selected === null) {
      // 用户取消了选择
      return;
    }
  } catch (error) {
    console.error("选择文件错误:", error);
    statusMsg.value = `选择文件失败: ${error.message || String(error)}`;
  }
}

async function convertFiles() {
  if (!txtFilePath.value) {
    statusMsg.value = "请先选择TXT文件";
    return;
  }

  if (!excelTemplatePath.value) {
    statusMsg.value = "请先选择Excel模板";
    return;
  }

  // 先让用户选择保存位置
  let outputPath;
  try {
    const saved = await save({
      filters: [
        {
          name: "Excel Files",
          extensions: ["xlsx"],
        },
      ],
      defaultPath: txtFilePath.value.replace(/\.txt$/i, "_converted.xlsx"),
    });

    if (!saved || typeof saved !== "string") {
      // 用户取消了保存
      statusMsg.value = "已取消保存";
      return;
    }

    outputPath = saved;
  } catch (error) {
    console.error("选择保存位置错误:", error);
    statusMsg.value = `选择保存位置失败: ${error.message || String(error)}`;
    return;
  }

  isProcessing.value = true;
  statusMsg.value = "正在转换，请稍候...";

  try {
    await invoke("convert_to_excel", {
      txtFilePath: txtFilePath.value,
      excelTemplatePath: excelTemplatePath.value,
      outputPath: outputPath,
    });

    statusMsg.value = `转换成功！文件已保存至: ${outputPath.split("/").pop()}`;
    txtFilePath.value = "";
    excelTemplatePath.value = "";
  } catch (error) {
    statusMsg.value = `转换失败: ${error}`;
  } finally {
    isProcessing.value = false;
  }
}
</script>

<template>
  <main class="container">
    <h1>TXT转Excel工具</h1>

    <div class="file-selector">
      <div class="file-item">
        <label>TXT文件：</label>
        <div class="file-info">
          <span v-if="txtFilePath" class="file-path">{{ txtFilePath.split("/").pop() }}</span>
          <span v-else class="file-placeholder">未选择文件</span>
        </div>
        <button @click="selectTxtFile" :disabled="isProcessing">选择TXT文件</button>
      </div>

      <div class="file-item">
        <label>Excel模板：</label>
        <div class="file-info">
          <span v-if="excelTemplatePath" class="file-path">{{ excelTemplatePath.split("/").pop() }}</span>
          <span v-else class="file-placeholder">未选择文件</span>
        </div>
        <button @click="selectExcelTemplate" :disabled="isProcessing">选择Excel模板</button>
      </div>
    </div>

    <div class="action-section">
      <button 
        class="convert-btn" 
        @click="convertFiles" 
        :disabled="isProcessing || !txtFilePath || !excelTemplatePath"
      >
        {{ isProcessing ? "转换中..." : "开始转换" }}
      </button>
    </div>

    <div class="status-message" :class="{ error: statusMsg.includes('失败') }">
      {{ statusMsg || "请选择TXT文件和Excel模板" }}
    </div>
  </main>
</template>

<style scoped>
.container {
  padding: 2rem;
  max-width: 800px;
  margin: 0 auto;
}

h1 {
  text-align: center;
  margin-bottom: 2rem;
  color: #396cd8;
}

.file-selector {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background-color: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.file-item label {
  min-width: 100px;
  font-weight: 500;
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-path {
  color: #396cd8;
  word-break: break-all;
}

.file-placeholder {
  color: #999;
  font-style: italic;
}

.action-section {
  display: flex;
  justify-content: center;
  margin-bottom: 1.5rem;
}

.convert-btn {
  padding: 0.8em 2em;
  font-size: 1.1em;
  font-weight: 600;
  background-color: #396cd8;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
}

.convert-btn:hover:not(:disabled) {
  background-color: #2952b8;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(57, 108, 216, 0.3);
}

.convert-btn:disabled {
  background-color: #ccc;
  cursor: not-allowed;
  transform: none;
}

.status-message {
  padding: 1rem;
  border-radius: 8px;
  background-color: #f0f0f0;
  text-align: center;
  min-height: 2rem;
  color: #333;
}

.status-message.error {
  background-color: #ffe0e0;
  color: #d32f2f;
}

button {
  padding: 0.6em 1.2em;
  font-size: 0.9em;
  border-radius: 6px;
  border: 1px solid #396cd8;
  background-color: white;
  color: #396cd8;
  cursor: pointer;
  transition: all 0.25s;
}

button:hover:not(:disabled) {
  background-color: #396cd8;
  color: white;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

@media (prefers-color-scheme: dark) {
  .file-item {
    background-color: #1e1e1e;
  }

  .status-message {
    background-color: #2a2a2a;
    color: #f6f6f6;
  }

  .status-message.error {
    background-color: #4a1e1e;
    color: #ff6b6b;
  }

  button {
    background-color: #1e1e1e;
    color: #396cd8;
    border-color: #396cd8;
  }

  button:hover:not(:disabled) {
    background-color: #396cd8;
    color: white;
  }
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
