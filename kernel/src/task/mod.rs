use crate::config::MAX_APP_NUM;
use crate::loader::*;
use crate::sync::UPSafeCell;
use crate::task::context::TaskContext;
use crate::task::task::{TaskControlBlock, TaskStatus};
use lazy_static::lazy_static;

mod context;
mod switch;
mod task;

pub struct TaskManager {
    app_num: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let app_num = get_app_num();

        let mut tasks = [TaskControlBlock {
            task_ctx: TaskContext::init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];

        for i in 0..app_num {
            tasks[i].task_ctx.goto_restore(init_app_ctx(i));
            tasks[i].task_status = TaskStatus::Ready;
        }

        TaskManager {
            app_num,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}
