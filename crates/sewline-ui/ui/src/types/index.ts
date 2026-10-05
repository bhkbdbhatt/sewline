export type StageStatus = 'pending' | 'running' | 'passed' | 'failed';

export interface Gate {
  id: string;
  name: string;
  policyRule: string;
  enforce: boolean;
  passed: boolean;
  attestationHash?: string;
}

export interface Stage {
  id: string;
  name: string;
  adapter: string;
  status: StageStatus;
  dependsOn: string[];
  gates: Gate[];
}

export interface PipelineExecution {
  id: string;
  name: string;
  complianceProfile: string;
  status: StageStatus;
  stages: Stage[];
}

export interface ApprovalRequest {
  approvalId: string;
  agentId: string;
  actionDescription: string;
  riskLevel: 'LOW' | 'MEDIUM' | 'HIGH' | 'CRITICAL';
  status: 'PENDING' | 'APPROVED' | 'REJECTED';
  contextPayloadJson: string;
}

export interface DsseAttestationEnvelope {
  payload: string;
  payloadType: string;
  signatures: Array<{
    keyid: string;
    sig: string;
  }>;
}