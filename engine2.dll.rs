#![allow(non_snake_case, non_upper_case_globals)]

pub mod CEntityComponentHelper {
    pub const m_flags: usize = 0x8;
    pub const m_pInfo: usize = 0x10;
    pub const m_nPriority: usize = 0x18;
    pub const m_pNext: usize = 0x20;
}

pub mod CEntityIOOutput {
    pub const m_Value: usize = 0x18;
}

pub mod CEntityIdentity {
    pub const m_nameStringableIndex: usize = 0x14;
    pub const m_name: usize = 0x18;
    pub const m_designerName: usize = 0x20;
    pub const m_flags: usize = 0x30;
    pub const m_worldGroupId: usize = 0x38;
    pub const m_fDataObjectTypes: usize = 0x3C;
    pub const m_PathIndex: usize = 0x40;
    pub const m_pPrev: usize = 0x58;
    pub const m_pNext: usize = 0x60;
    pub const m_pPrevByClass: usize = 0x68;
    pub const m_pNextByClass: usize = 0x70;
}

pub mod CEntityInstance {
    pub const m_iszPrivateVScripts: usize = 0x8;
    pub const m_pEntity: usize = 0x10;
    pub const m_CScriptComponent: usize = 0x28;
}

pub mod CNetworkVarChainer {
    pub const m_PathIndex: usize = 0x20;
}

pub mod CScriptComponent {
    pub const m_scriptClassName: usize = 0x30;
}

pub mod EngineLoopState_t {
    pub const m_nPlatWindowWidth: usize = 0x18;
    pub const m_nPlatWindowHeight: usize = 0x1C;
    pub const m_nRenderWidth: usize = 0x20;
    pub const m_nRenderHeight: usize = 0x24;
}

pub mod EntComponentInfo_t {
    pub const m_pName: usize = 0x0;
    pub const m_pCPPClassname: usize = 0x8;
    pub const m_pNetworkDataReferencedDescription: usize = 0x10;
    pub const m_pNetworkDataReferencedPtrPropDescription: usize = 0x18;
    pub const m_nRuntimeIndex: usize = 0x20;
    pub const m_nFlags: usize = 0x24;
    pub const m_pBaseClassComponentHelper: usize = 0x60;
}

pub mod EventAdvanceTick_t {
    pub const m_nCurrentTick: usize = 0x30;
    pub const m_nCurrentTickThisFrame: usize = 0x34;
    pub const m_nTotalTicksThisFrame: usize = 0x38;
    pub const m_nTotalTicks: usize = 0x3C;
}

pub mod EventAppShutdown_t {
    pub const m_nDummy0: usize = 0x0;
}

pub mod EventClientFrameSimulate_t {
    pub const m_LoopState: usize = 0x0;
    pub const m_flRealTime: usize = 0x28;
    pub const m_flFrameTime: usize = 0x2C;
}

pub mod EventClientOutput_t {
    pub const m_LoopState: usize = 0x0;
    pub const m_flRenderTime: usize = 0x28;
    pub const m_flRealTime: usize = 0x2C;
    pub const m_flRenderFrameTimeUnbounded: usize = 0x30;
    pub const m_bRenderOnly: usize = 0x34;
}

pub mod EventClientPollInput_t {
    pub const m_LoopState: usize = 0x0;
    pub const m_flRealTime: usize = 0x28;
}

pub mod EventClientPollNetworking_t {
    pub const m_nTickCount: usize = 0x0;
}

pub mod EventClientPostOutput_t {
    pub const m_LoopState: usize = 0x0;
    pub const m_flRenderTime: usize = 0x28;
    pub const m_flRenderFrameTime: usize = 0x30;
    pub const m_flRenderFrameTimeUnbounded: usize = 0x34;
    pub const m_bRenderOnly: usize = 0x38;
}

pub mod EventClientPreOutput_t {
    pub const m_LoopState: usize = 0x0;
    pub const m_flRenderTime: usize = 0x28;
    pub const m_flRenderFrameTime: usize = 0x30;
    pub const m_flRenderFrameTimeUnbounded: usize = 0x38;
    pub const m_flRealTime: usize = 0x40;
    pub const m_bRenderOnly: usize = 0x44;
}

pub mod EventClientProcessGameInput_t {
    pub const m_LoopState: usize = 0x0;
    pub const m_flRealTime: usize = 0x28;
    pub const m_flFrameTime: usize = 0x2C;
}

pub mod EventClientProcessInput_t {
    pub const m_LoopState: usize = 0x0;
    pub const m_flRealTime: usize = 0x28;
}

pub mod EventClientSceneSystemThreadStateChange_t {
    pub const m_bThreadsActive: usize = 0x0;
}

pub mod EventClientSendInput_t {
    pub const m_bFinalClientCommandTick: usize = 0x0;
    pub const m_nAdditionalClientCommandsToCreate: usize = 0x4;
}

pub mod EventFrameBoundary_t {
    pub const m_flFrameTime: usize = 0x0;
}

pub mod EventPostAdvanceTick_t {
    pub const m_nCurrentTick: usize = 0x30;
    pub const m_nCurrentTickThisFrame: usize = 0x34;
    pub const m_nTotalTicksThisFrame: usize = 0x38;
    pub const m_nTotalTicks: usize = 0x3C;
}

pub mod EventPostDataUpdate_t {
    pub const m_nCount: usize = 0x0;
}

pub mod EventPreDataUpdate_t {
    pub const m_nCount: usize = 0x0;
}

pub mod EventProfileStorageAvailable_t {
    pub const m_nSplitScreenSlot: usize = 0x0;
}

pub mod EventSetTime_t {
    pub const m_LoopState: usize = 0x0;
    pub const m_nClientOutputFrames: usize = 0x28;
    pub const m_flRealTime: usize = 0x30;
    pub const m_flRenderTime: usize = 0x38;
    pub const m_flRenderFrameTime: usize = 0x40;
    pub const m_flRenderFrameTimeUnbounded: usize = 0x48;
    pub const m_flRenderFrameTimeUnscaled: usize = 0x50;
    pub const m_flTickRemainder: usize = 0x58;
}

pub mod EventSimpleLoopFrameUpdate_t {
    pub const m_LoopState: usize = 0x0;
    pub const m_flRealTime: usize = 0x28;
    pub const m_flFrameTime: usize = 0x2C;
}

pub mod EventSimulate_t {
    pub const m_LoopState: usize = 0x0;
    pub const m_bFirstTick: usize = 0x28;
    pub const m_bLastTick: usize = 0x29;
}