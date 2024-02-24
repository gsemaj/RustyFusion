use std::time::{Duration, SystemTime};

use crate::{
    defines::{SIZEOF_QUESTFLAG_NUMBER, SIZEOF_RQUEST_SLOT},
    enums::*,
    error::{FFError, FFResult, Severity},
    net::packet::sRunningQuest,
    tabledata::tdata_get,
};

#[derive(Debug)]
pub struct MissionDefinition {
    pub mission_id: i32,
    pub mission_name: String,
    pub task_ids: Vec<i32>,
    pub mission_type: MissionType,
}

#[derive(Debug)]
pub struct TaskDefinition {
    pub task_id: i32,                 // m_iHTaskID
    pub mission_id: i32,              // m_iHMissionID
    pub task_type: TaskType,          // m_iHTaskType
    pub success_task_id: Option<i32>, // m_iSUOutgoingTask
    pub fail_task_id: Option<i32>,    // m_iFOutgoingTask

    // prerequisites
    pub giver_npc_type: Option<i32>,            // m_iHNPCID
    pub prereq_completed_mission_ids: Vec<i32>, // m_iCSTReqMission
    pub prereq_nano_ids: Vec<i16>,              // m_iCSTRReqNano
    pub prereq_level: Option<i16>,              // m_iCTRReqLvMin
    pub prereq_guide: Option<PlayerGuide>,      // m_iCSTReqGuide
    pub prereq_items: Vec<(i16, usize)>,        // m_iCSTItemID, m_iCSTItemNumNeeded
    pub prereq_running_task_id: Option<i32>,    // m_iCSTTrigger

    // win conditions
    pub time_limit: Option<Duration>,          // m_iCSUCheckTimer
    pub destination_npc_type: Option<i32>,     // m_iHTerminatorNPCID
    pub destination_map_num: Option<u32>,      // m_iRequireInstanceID
    pub req_items: Vec<(i16, usize)>,          // m_iCSUItemID, m_iCSUItemNumNeeded
    pub req_defeat_enemies: Vec<(i32, usize)>, // m_iCSUEnemyID, m_iCSUNumToKill
    pub escort_npc_type: Option<i32>,          // m_iCSUDEFNPCID
}

#[derive(Debug, Clone)]
pub struct Task {
    task_id: i32,
    pub remaining_enemies: Vec<(i32, usize)>,
    pub fail_time: Option<SystemTime>,
    pub completed: bool,
}
impl Task {
    pub fn get_task_def(&self) -> FFResult<&TaskDefinition> {
        tdata_get().get_task_definition(self.task_id)
    }

    pub fn get_mission_def(&self) -> FFResult<&MissionDefinition> {
        let task_def = self.get_task_def()?;
        let mission_def = tdata_get().get_mission_definition(task_def.mission_id)?;
        Ok(mission_def)
    }
}
impl From<&TaskDefinition> for Task {
    fn from(task_def: &TaskDefinition) -> Self {
        Task {
            task_id: task_def.task_id,
            remaining_enemies: task_def.req_defeat_enemies.clone(),
            fail_time: task_def.time_limit.map(|d| SystemTime::now() + d),
            completed: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MissionJournal {
    pub current_nano_mission: Option<Task>,
    pub current_guide_mission: Option<Task>,
    pub current_world_missions: [Option<Task>; 4],
    pub active_mission_slot: Option<usize>,
    completed_mission_flags: [i64; SIZEOF_QUESTFLAG_NUMBER as usize],
}
impl MissionJournal {
    fn get_task_iter(&self) -> impl Iterator<Item = &Task> {
        let mut tasks = Vec::new();
        if let Some(task) = &self.current_nano_mission {
            tasks.push(task);
        }
        if let Some(task) = &self.current_guide_mission {
            tasks.push(task);
        }
        tasks.extend(
            self.current_world_missions
                .iter()
                .filter_map(Option::as_ref),
        );
        tasks.into_iter()
    }

    fn get_task_iter_mut(&mut self) -> impl Iterator<Item = &mut Task> {
        let mut tasks = Vec::new();
        if let Some(task) = &mut self.current_nano_mission {
            tasks.push(task);
        }
        if let Some(task) = &mut self.current_guide_mission {
            tasks.push(task);
        }
        tasks.extend(
            self.current_world_missions
                .iter_mut()
                .filter_map(Option::as_mut),
        );
        tasks.into_iter()
    }

    fn get_current_task_by_idx(&self, idx: usize) -> Option<&Task> {
        match idx {
            0 => self.current_nano_mission.as_ref(),
            1 => self.current_guide_mission.as_ref(),
            _ => self.current_world_missions[idx - 2].as_ref(),
        }
    }

    pub fn get_mission_flags(&self) -> [i64; SIZEOF_QUESTFLAG_NUMBER as usize] {
        self.completed_mission_flags
    }

    pub fn get_running_quests(&self) -> [sRunningQuest; SIZEOF_RQUEST_SLOT as usize] {
        let mut running_quests = [sRunningQuest::default(); SIZEOF_RQUEST_SLOT as usize];
        for (i, quest) in running_quests.iter_mut().enumerate().take(6) {
            let task = self.get_current_task_by_idx(i);
            if let Some(task) = task {
                let task_def = task.get_task_def().unwrap();
                quest.m_aCurrTaskID = task_def.task_id;
                for (j, (npc_id, count)) in task.remaining_enemies.iter().enumerate() {
                    quest.m_aKillNPCID[j] = *npc_id;
                    quest.m_aKillNPCCount[j] = *count as i32;
                }
                for (j, (item_id, count)) in task_def.req_items.iter().enumerate() {
                    quest.m_aNeededItemID[j] = *item_id as i32;
                    quest.m_aNeededItemCount[j] = *count as i32;
                }
            }
        }
        running_quests
    }

    pub fn get_active_mission_id(&self) -> Option<i32> {
        let idx = self.active_mission_slot?;
        let active_task = self.get_current_task_by_idx(idx)?;
        let task_def = active_task.get_task_def().unwrap();
        Some(task_def.mission_id)
    }

    pub fn get_current_task_ids(&self) -> Vec<i32> {
        let mut task_ids = Vec::new();
        for task in self.get_task_iter() {
            let task_def = task.get_task_def().unwrap();
            task_ids.push(task_def.task_id);
        }
        task_ids
    }

    pub fn is_mission_completed(&self, mission_id: i32) -> FFResult<bool> {
        const MAX_MISSION_ID: i32 = SIZEOF_QUESTFLAG_NUMBER as i32 * 64;
        match mission_id {
            1..=MAX_MISSION_ID => {
                let offset = mission_id - 1;
                let flags_idx = offset / 32;
                let bit_idx = offset % 32;
                Ok((self.completed_mission_flags[flags_idx as usize] & (1 << bit_idx)) != 0)
            }
            _ => Err(FFError::build(
                Severity::Warning,
                format!("Invalid mission ID {}", mission_id),
            )),
        }
    }

    pub fn start_task(&mut self, task: Task) -> FFResult<()> {
        let mission_def = task.get_mission_def()?;
        let mission_existing_task = self
            .get_task_iter_mut()
            .find(|t| t.get_task_def().unwrap().mission_id == mission_def.mission_id);
        if let Some(existing_task) = mission_existing_task {
            if !existing_task.completed {
                return Err(FFError::build(
                    Severity::Warning,
                    format!(
                        "Tried to start task {} while task {} for mission {} is in progress",
                        task.task_id, existing_task.task_id, mission_def.mission_id
                    ),
                ));
            }
            *existing_task = task; // replace existing task
        } else {
            let slot = match mission_def.mission_type {
                MissionType::Unknown => {
                    return Err(FFError::build(
                        Severity::Warning,
                        format!(
                            "Tried to start task {} for unknown mission type",
                            task.task_id
                        ),
                    ))
                }
                MissionType::Guide => &mut self.current_guide_mission,
                MissionType::Nano => &mut self.current_nano_mission,
                MissionType::Normal => self
                    .current_world_missions
                    .iter_mut()
                    .find(|slot| slot.is_none())
                    .ok_or(FFError::build(
                        Severity::Warning,
                        "No empty world mission slots".to_string(),
                    ))?,
            };
            *slot = Some(task);
        }
        Ok(())
    }
}

impl Default for sRunningQuest {
    fn default() -> Self {
        sRunningQuest {
            m_aCurrTaskID: 0,
            m_aKillNPCID: [0; 3],
            m_aKillNPCCount: [0; 3],
            m_aNeededItemID: [0; 3],
            m_aNeededItemCount: [0; 3],
        }
    }
}
