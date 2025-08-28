#!/bin/bash

# 定义要执行的 Git 命令
GIT_COMMAND="git push"

# 定义重试的间隔时间（秒）
RETRY_DELAY=180

# 定义一个日志文件，用于记录每次尝试的结果
LOG_FILE="git_push.log"

# 无限循环，直到 git push 成功
while true; do
    echo "[$RETRY_DELAY 秒后重试] 正在尝试执行 '$GIT_COMMAND'..." | tee -a "$LOG_FILE"
    
    # 执行 Git 命令，并将标准输出和标准错误都重定向到日志文件
    # 并判断命令的退出状态。0 表示成功，非0 表示失败
    if $GIT_COMMAND >> "$LOG_FILE" 2>&1; then
        echo "✅ Git Push 成功！脚本已结束。" | tee -a "$LOG_FILE"
        break
    else
        echo "❌ Git Push 失败，等待 $RETRY_DELAY 秒后重试..." | tee -a "$LOG_FILE"
        sleep "$RETRY_DELAY"
    fi
done

exit 0