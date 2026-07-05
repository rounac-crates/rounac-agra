#![doc = r#"Helper types for use with serde try_from and into."#]

use serde::{Deserialize, Serialize};

#[doc = r#"Specifies the Subject and the associated objects of the AccessAssessment."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AccessAssessmentResultTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AccessAssessmentId: Option<Vec<crate::v5_0::types::AccessAssessmentIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Assessment: Option<Vec<crate::v5_0::types::AccessAssessmentType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ActionCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::ActionCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::ActivityCommandBaseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ActionPlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActionPlanCommandId: Option<crate::v5_0::types::ActionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ActivityActorIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityId: Option<crate::v5_0::types::CapabilityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ActivityChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EffectId: Option<crate::v5_0::types::EffectIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActionId: Option<crate::v5_0::types::ActionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TaskId: Option<crate::v5_0::types::TaskIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ResponseId: Option<crate::v5_0::types::ResponseIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityCommand: Option<crate::v5_0::types::CapabilityCommandBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SupportingCapabilityCommand: Option<crate::v5_0::types::SupportCapabilityCommandBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VehicleSettings: Option<crate::v5_0::types::VehicleCommandDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommAction: Option<crate::v5_0::types::CommDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductTaskId: Option<crate::v5_0::types::TaskIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ActivityPlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActivityPlanCommandId: Option<crate::v5_0::types::ActivityPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ActivityPlansIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteActivityPlanId: Option<crate::v5_0::types::RouteActivityPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitActivityPlanId: Option<crate::v5_0::types::OrbitActivityPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActivityPlanId: Option<crate::v5_0::types::ActivityPlanIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ActivitySourceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Requirement: Option<crate::v5_0::choices::RequirementInstanceIdChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Mdf: Option<crate::v5_0::types::MdfReferenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActivityPlan: Option<crate::v5_0::types::ActivityPlanReferenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AssociatedMessage: Option<crate::v5_0::choices::AssociatedMessageSourceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ActivityTriggerTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TransitionTime: Option<crate::v5_0::types::DateTimeRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DistanceToEndPoint: Option<crate::v5_0::common::DistanceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AirSampleCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::AirSampleCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::ActivityCommandBaseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AirTargetVolumeCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirVolume: Option<crate::v5_0::types::AirVolumeCueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AirTargetVolumeTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirVolumeSensorReferenced: Option<crate::v5_0::types::AirVolumeSensorReferencedType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirVolumeLocation: Option<crate::v5_0::types::ZoneType>,
}

#[doc = r#"See the annotation in the associated message airfield status data."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AirfieldReferenceIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AmtiCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::AmtiCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::AmtiActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AmtiTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirVolumeSensorReferenced: Option<crate::v5_0::types::AirVolumeSensorReferencedType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirVolumeLocation: Option<crate::v5_0::types::ZoneType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AntennaResourceChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AntennaResourceTypeId: Option<crate::v5_0::types::AntennaResourceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AntennaResourceInstanceId: Option<crate::v5_0::types::AntennaResourceIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AoCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::AoCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::AoActivityCommandType>,
}

#[doc = r#"Utilized by RF_ResourceAllocation and RF_ResourceAllocationRequest to indicate the spatial coverage needed for the Activity requesting resources."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AperturePointingOptionsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FieldOfRegard: Option<crate::v5_0::types::FieldOfRegardType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InstallationIndex: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequestBeamPointingReference: Option<crate::v5_0::choices::BeamPointingReferenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequestEntityReferenceId: Option<crate::v5_0::types::EntityIdType>,
}

#[doc = r#"Indicates the Approach Angle either in Azimuth / Elevation or a unit vector relative to the body coordinate system of the target."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ApproachAngleTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AzEl: Option<crate::v5_0::types::AzElReferenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Relative: Option<crate::v5_0::types::UnitVectorType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ApprovalRequestItemReferenceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlanApproval: Option<crate::v5_0::choices::PlanReferenceIdChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementExecutionApproval: Option<crate::v5_0::types::ApprovalRequestItemType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanActivationApproval: Option<Vec<crate::v5_0::types::MissionPlanActivationCommandType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ArchiveRequestTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Time: Option<chrono::DateTime<chrono::Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NumberOfDays: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpaceNeeded: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ManuallyDeleted: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NumberOfMissions: Option<u32>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AreaChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Polygon: Option<crate::v5_0::types::PolygonType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ellipse: Option<crate::v5_0::types::LocatedEllipseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Rectangle: Option<crate::v5_0::types::LocatedRectangleType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SlantRangeArea: Option<crate::v5_0::types::SlantRangeAreaType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AssessmentRequestTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommPointingPlan: Option<crate::v5_0::types::CommPointingPlanRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityUtilization: Option<crate::v5_0::types::CapabilityUtilizationRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteDeconfliction: Option<crate::v5_0::types::RouteDeconflictionRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteVulnerabilityMetrics: Option<crate::v5_0::types::RouteVulnerabilityMetricsRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteThreatAssessment: Option<crate::v5_0::types::ThreatAssessmentRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TargetMobility: Option<crate::v5_0::types::TargetMobilityRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VehicleThreatAssessment: Option<crate::v5_0::types::VehicleThreatAssessmentRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ThreatNominationAssessment: Option<crate::v5_0::types::ThreatNominationAssessmentRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AchievabilityAssessment: Option<crate::v5_0::types::AchievabilityAssessmentRequestPet>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AssessmentTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommPointingPlan: Option<crate::v5_0::types::CommPointingPlanAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityUtilization: Option<crate::v5_0::types::CapabilityUtilizationAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteDeconfliction: Option<crate::v5_0::types::RouteDeconflictionAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteVulnerabilityMetrics: Option<crate::v5_0::types::RouteVulnerabilityMetricsAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteThreatAssessment: Option<crate::v5_0::types::RouteThreatAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TargetMobility: Option<crate::v5_0::types::TargetMobilityAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VehicleThreatAssessment: Option<crate::v5_0::types::VehicleThreatAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ThreatNominationAssessment: Option<crate::v5_0::types::ThreatNominationAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AchievabilityAssessment: Option<crate::v5_0::types::AchievabilityAssessmentPet>,
}

#[doc = r#"Provides a choice between a System and an Entity."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AssetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
}

#[doc = r#"Used to reference an associated message or message element which has generated an Activity.."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AssociatedMessageSourceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SettingsCommandId: Option<crate::v5_0::types::CommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RfResouceRequestIdandOption: Option<Vec<crate::v5_0::types::ResourceRequestIdAndOptionType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActiveScanScheduleProfileIndex: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EmitterPriorityBinDetail: Option<Vec<crate::v5_0::types::SharedSourceEmitterPriorityBinDetailType>>,
}

#[doc = r#"Encoding types for CVEnumISMatomicEnergyMarkings Version 1 controlled vocabulary enumerations.  Derived from the CVEnumISMatomicEnergyMarkings.xml CVE.(U) All currently valid Atomic Energy information markings from the published register
						   PERMISSIBLE VALUES

						   The permissible values for this simple type are defined in the Controlled Value Enumeration:

						   CVEnumISMatomicEnergyMarkings.xml"#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AtomicEnergyMarkingsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Markings: Option<crate::v5_0::enums::AtomicEnergyMarkingsEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SigmaMarkings: Option<crate::v5_0::enums::AtomicEnergySigmaMarkingsEnum>,
}

#[doc = r#"An atomic primitive value."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AtomicValueTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BooleanValue: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByteValue: Option<i8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnsignedByteValue: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ShortValue: Option<i16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnsignedShortValue: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IntValue: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnsignedIntValue: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LongValue: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FloatValue: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DoubleValue: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DateTimeValue: Option<chrono::DateTime<chrono::Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub DurationValue: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::naive_time_opt")]
	pub TimeValue: Option<chrono::NaiveTime>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StringValueCaseSensitive: Option<crate::v5_0::common::QueryString4096Type>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StringValueCaseInsensitive: Option<crate::v5_0::common::QueryString4096Type>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EnumValue: Option<crate::v5_0::common::SchemaComponentNameType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub HexBinaryValue: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UuidValue: Option<uuid::Uuid>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AutonomousActionStatusChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutonomousPlanningActionStatus: Option<Vec<crate::v5_0::types::AutonomousPlanningActionStatusType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AlertOnly: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct AvailableFuelTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Fuel: Option<crate::v5_0::common::MassType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Percent: Option<crate::v5_0::common::PercentType>,
}

#[doc = r#"Beam pointing reference types which includes Antenna, Body, Inertial, or Geodetic."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct BeamPointingReferenceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Antenna: Option<crate::v5_0::types::AzElBeamPointingType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Body: Option<crate::v5_0::types::AzElBeamPointingType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Inertial: Option<crate::v5_0::types::AzElBeamPointingWithRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Geodetic: Option<crate::v5_0::types::Point2DType>,
}

#[doc = r#"This type defines limited Beam Shaping Direction to the Antenna."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct BeamShapingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BeamWeighting: Option<crate::v5_0::types::BeamWeightingType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BeamWidth: Option<crate::v5_0::types::BeamWidthType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct BlankingSourceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SubsystemId: Option<crate::v5_0::types::SubsystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DigitalPayload: Option<crate::v5_0::choices::DigitalFunctionType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct BlueVehicleTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityIdentity: Option<crate::v5_0::types::IdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Model: Option<crate::v5_0::common::VisibleString32Type>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct BoundaryTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Polygon: Option<crate::v5_0::types::PolygonType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ellipse: Option<crate::v5_0::types::LocatedEllipseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CapabilityAssessmentActionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlannedTaskId: Option<crate::v5_0::types::TaskIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EstimatedCapabilityCommand: Option<crate::v5_0::types::CapabilityCommandBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EstimatedSupportingCapabilityCommand: Option<crate::v5_0::types::SupportCapabilityCommandBaseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CapabilityAssessmentActivityTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EstimatedCapabilityActivity: Option<crate::v5_0::types::ActivityBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EstimatedSupportingCapabilityReport: Option<crate::v5_0::types::SupportCapabilityStatusBaseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CapabilityCrossReferenceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityId: Option<crate::v5_0::types::CapabilityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SupportCapabilityId: Option<crate::v5_0::types::SupportCapabilityIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CargoDeliveryTaskTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Pickup: Option<crate::v5_0::types::CargoTransitionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Dropoff: Option<Vec<crate::v5_0::types::CargoTransitionType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CargoLocationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Point: Option<crate::v5_0::types::Point2DType>,
}

#[doc = r#"See the annotation in the associated message carrier status data."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CarrierReferenceIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
}

#[doc = r#"The CharacterizationChoiceType is a choice type that allows the user to select a specific type of characterization of the object."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CharacterizationChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Frequency: Option<crate::v5_0::types::FrequencyParamsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IrImage: Option<crate::v5_0::types::IrImageParamsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MetricObservations: Option<crate::v5_0::types::MetricParamsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NarrowbandSoi: Option<crate::v5_0::types::NarrowbandSoiParamsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpticalImage: Option<crate::v5_0::types::OpticalImageParamsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Rcs: Option<crate::v5_0::types::RcsParamsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VisMag: Option<crate::v5_0::types::VisMagParamsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WidebandSoi: Option<crate::v5_0::types::WidebandSoiParamsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Photometry: Option<crate::v5_0::types::PhotometryParamsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ColorPhotometry: Option<crate::v5_0::types::ColorPhotometryParamsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CharacterizationObjectiveTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PhenomenologyCollection: Option<crate::v5_0::types::CharacterizationOptionsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StabilityAndOrientationAssessment: Option<crate::v5_0::types::StabilityCharacterizationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StructureAssessment: Option<crate::v5_0::choices::StructureAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IdentificationVerification: Option<crate::v5_0::types::IdentificationVerificationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OperationsChanges: Option<crate::v5_0::types::SatelliteOperationsChangesCharacterizationType>,
}

#[doc = r#"Used to specify the choice Civil Path Terminator Type associated with the End Point, to include its specific parameters needed."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CivilPathTerminatorTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AfArcToFix: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CaCourseToAltitude: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CdCourseToDmedistance: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CfCourseToFix: Option<crate::v5_0::types::CfCourseToFixType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CiCourseToIntercept: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CrCourseToRadial: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DfDirectToFix: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FaTrackToAltitude: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FcTrackFromFixToDistanceAlongTrack: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FdTrackFromFixToDmedistance: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FmFixToManualTermination: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub HaHoldingWithAltitudeTermination: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub HfHoldingWithFixTermination: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub HmHoldingWithManualTermination: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IfInitialFix: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PiProcedureTurnToIntercept: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RfRadiusToFix: Option<crate::v5_0::types::RfRadiusToFixType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TfTrackToFix: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VaHeadingToAltitude: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VdHeadingToDmedistanceTermination: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ViHeadingToIntercept: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VmHeadingToManual: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VrHeadingToRadialTermination: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"Indicates the orientation of the orbital plane in space and of the orbit within its plane."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CoeOrientationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NonEquatorialOrbit: Option<crate::v5_0::types::CoeNonEquatorialOrientationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EquatorialOrbit: Option<crate::v5_0::types::CoeEquatorialOrientationType>,
}

#[doc = r#"Indicates the set of classic orbital elements (COE) describing a spacecraft's position in an orbit.  Elements describing the size, shape and other characteristics of the orbit are in other types."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CoePositionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MeanAnomaly: Option<crate::v5_0::common::AnglePositiveType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ArgumentOfLatitude: Option<crate::v5_0::common::AnglePositiveType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TrueLongitude: Option<crate::v5_0::common::AnglePositiveType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ComintCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::ComintCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::ComintActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ComintSubCapabilityDetailsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Acquisition: Option<crate::v5_0::types::ComintAcquisitionTargetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DirectionFinding: Option<crate::v5_0::types::ComintDirectionFindingType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Identification: Option<crate::v5_0::types::ComintIdentificationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Geolocation: Option<crate::v5_0::types::ComintGeolocationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Measurement: Option<crate::v5_0::types::ComintMeasurementType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ComintSubcapabilityChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Acquisition: Option<crate::v5_0::types::ComintSubcapabilityAcquisitionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Identification: Option<crate::v5_0::types::ComintSubcapabilityIdentificationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Geolocation: Option<crate::v5_0::types::ComintSubcapabilityGeolocationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Measurement: Option<crate::v5_0::types::ComintSubcapabilityMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DataCollect: Option<crate::v5_0::types::ComintSubcapabilityDataCollectType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ComintSubcapabilityTargetLocationDataTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DwellFov: Option<crate::v5_0::types::NedConeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PointTarget: Option<crate::v5_0::types::PointTargetType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ComintTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EmitterType: Option<crate::v5_0::types::EmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificEmitter: Option<crate::v5_0::types::SpecificEmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalDescription: Option<crate::v5_0::types::SignalSummaryType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalId: Option<crate::v5_0::types::SignalIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TargetClass: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CommLinkDetailsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FullDuplex: Option<crate::v5_0::types::LinkRatesType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TxSimplex: Option<crate::v5_0::common::DataRateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub HalfDuplex: Option<crate::v5_0::types::LinkRatesType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RxSimplex: Option<crate::v5_0::common::DataRateType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CommRangeDelayChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Active: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub Passive: Option<chrono::TimeDelta>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CommRelayCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::CommRelayCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::ActivityCommandBaseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CommTerminalCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::CommTerminalCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::CommTerminalActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CommandResponseTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirSample: Option<crate::v5_0::types::AirSampleCommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Amti: Option<crate::v5_0::types::AmtiCommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ao: Option<crate::v5_0::types::AoCommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Comint: Option<crate::v5_0::types::ComintCommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommRelay: Option<crate::v5_0::types::CommRelayCommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ea: Option<crate::v5_0::types::EaCommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Esm: Option<crate::v5_0::types::EsmCommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Po: Option<crate::v5_0::types::PoCommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Sar: Option<crate::v5_0::types::SarCommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Smti: Option<crate::v5_0::types::SmtiCommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Strike: Option<crate::v5_0::choices::StrikeWeaponCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeatherRadar: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"A comparable atomic primitive value."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ComparableAtomicValueTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByteValue: Option<i8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnsignedByteValue: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ShortValue: Option<i16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnsignedShortValue: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IntValue: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnsignedIntValue: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LongValue: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FloatValue: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DoubleValue: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DateTimeValue: Option<chrono::DateTime<chrono::Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub DurationValue: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::naive_time_opt")]
	pub TimeValue: Option<chrono::NaiveTime>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StringValue: Option<crate::v5_0::common::QueryString4096Type>,
}

#[doc = r#"The choice type for component configuration type.  Used within the component configuration type to allow representing a recursive tree structure of components/units/parts within a component."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ComponentConfigurationChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ComponentConfigurationList: Option<Vec<crate::v5_0::types::ComponentConfigurationPet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Terminator: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"Indicates an externally defined identifier for a type of component element that is specific to a system, subsystem, component, or service."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ComponentElementIdentifierChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ComponentElementNumericIdentifier: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ComponentElementKey: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ComponentResourceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SelectRfComponents: Option<crate::v5_0::types::SelectRfComponentResourceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SelectDigitalComponents: Option<crate::v5_0::types::SelectDigitalComponentResourceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ConfigurationParameterValueRestrictionsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Enumeration: Option<Vec<crate::v5_0::common::VisibleString32Type>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Range: Option<crate::v5_0::types::ConfigurationParameterRangeType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ContingencyPathSpacingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub Duration: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Distance: Option<crate::v5_0::common::DistanceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Endpoints: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ControlInterfacesControlTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionControl: Option<crate::v5_0::types::MissionControlInterfacesCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityControl: Option<Vec<crate::v5_0::types::ControlInterfacesCapabilityControlType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ControlSourcesControlTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ControllerSystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityControl: Option<Vec<crate::v5_0::types::ControlSourcesCapabilityControlType>>,
}

#[doc = r#"Choice indicating transfer of control, or the details of the new control status."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ControlTransferChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InProgress: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NewMissionControl: Option<crate::v5_0::types::ControlStatusMissionControlType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CountryCodeTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CountryName: Option<crate::v5_0::enums::GencCountryNameEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OperatorUniqueAssetName: Option<crate::v5_0::enums::OperatorUniqueNameEnum>,
}

#[doc = r#"This type represents the source of a key."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CryptoKeySourceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub KeyFileId: Option<crate::v5_0::types::FileLocationIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ds101: Option<crate::v5_0::enums::CryptoDs101Enum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Otam: Option<crate::v5_0::enums::CryptoOtamEnum>,
}

#[doc = r#"This type indicates the particular kind of CSO and provides additional details about the characteristics of the event."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct CsoDetailsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DockingEvent: Option<crate::v5_0::types::OrbitalDockingEventType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UndockingEvent: Option<crate::v5_0::types::OrbitalUndockingEventType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SeparationEvent: Option<crate::v5_0::types::OrbitalSeparationEventType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RendezvousEvent: Option<crate::v5_0::types::OrbitalRendezvousEventType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProximityOperationsEvent: Option<crate::v5_0::types::OrbitalProximityOperationsEventType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DamageEstimateTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TaskId: Option<crate::v5_0::types::TaskIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Target: Option<crate::v5_0::choices::TargetType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DamageObjectClassTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlatformType: Option<crate::v5_0::types::PlatformIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificType: Option<crate::v5_0::types::SpecificIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Human: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DamageSubjectTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TaskId: Option<Vec<crate::v5_0::types::TaskIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Target: Option<crate::v5_0::choices::TargetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Point: Option<crate::v5_0::types::Point2DType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DamageTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FunctionalDamage: Option<crate::v5_0::types::DamagedFunctionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub HumanCasualty: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub HumanInjury: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DamagedObjectIdentityTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlatformType: Option<crate::v5_0::types::PlatformIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificType: Option<crate::v5_0::types::SpecificIdentityType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DamagedObjectTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NonEntity: Option<crate::v5_0::types::DamagedObjectNonEntityType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DataDeleteChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductMetadataId: Option<crate::v5_0::types::ProductMetadataIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductLocationId: Option<crate::v5_0::types::ProductLocationIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FileMetadataId: Option<crate::v5_0::types::FileMetadataIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FileLocationId: Option<crate::v5_0::types::FileLocationIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityId: Option<Vec<crate::v5_0::types::CapabilityIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ComponentId: Option<Vec<crate::v5_0::types::ComponentIdType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DataManagementCategoryTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SaveMissionId: Option<crate::v5_0::types::MissionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SaveAs: Option<crate::v5_0::types::DataManagementSaveAsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DeleteMissionId: Option<crate::v5_0::types::MissionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ImportData: Option<crate::v5_0::types::DataManagementImportExportType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ExportData: Option<crate::v5_0::types::DataManagementImportExportType>,
}

#[doc = r#"Identifies the destination data port through which this message will be transmitted. This specifies the specific location the data shall transition through."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DataPortTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UserIoId: Option<crate::v5_0::types::CommUserIoIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RfUserLinkId: Option<crate::v5_0::types::CommUserLinkIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CryptoId: Option<crate::v5_0::types::SupportCapabilityIdType>,
}

#[doc = r#"Description of the data producer or producers that are expected to respond to a data update request."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DataUpdateOriginatorTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AllProducers: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Source: Option<crate::v5_0::types::DataUpdateSourceType>,
}

#[doc = r#"Parameters describing the specific kind of data that is being requested."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DataUpdateRequestTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Query: Option<crate::v5_0::types::QueryMessageType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub QuerySpecificData: Option<crate::v5_0::types::QuerySpecificDataPet>,
}

#[doc = r#"Identifier of a Digital Payload or a MutiFunctionArray (MFA)."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DigitalFunctionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DigitalPayloadCapabilityId: Option<crate::v5_0::types::CapabilityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MfaSupportCapabilityId: Option<crate::v5_0::types::SupportCapabilityIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DisseminationProductTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByReference: Option<crate::v5_0::choices::ProductReferenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByType: Option<crate::v5_0::types::DisseminationByType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DmpiPatternChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub XbyYPattern: Option<crate::v5_0::types::DmpiXbyYPatternType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RadialPattern: Option<crate::v5_0::types::DmpiRadialPatternType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FreePatternDmpiId: Option<Vec<crate::v5_0::types::DmpiIdType>>,
}

#[doc = r#"Indicates the target of the DMPI. The target can be specified by location or by identity. This allows DMPI targets to be specified based on target types."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DmpiTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByLocation: Option<crate::v5_0::types::DmpiLocationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByIdentity: Option<crate::v5_0::types::IdentityType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DmpiViolationChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpLineId: Option<crate::v5_0::types::OpLineIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZoneId: Option<crate::v5_0::types::OpZoneIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolumeId: Option<crate::v5_0::types::OpVolumeIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DoorCommandChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DoorState: Option<crate::v5_0::enums::DoorCommandEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CloseOnHungStore: Option<bool>,
}

#[doc = r#"A choice of drag coefficient to use. It is a choice between a simple drag coefficient and VCM drag parameters."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct DragCoefficientChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VcmDragParameters: Option<crate::v5_0::types::VcmDragParametersType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DragCoefficientValue: Option<f64>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EaCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::EaCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::EaActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EaDetailsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Assigned: Option<crate::v5_0::types::EaDetailsAssignedType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Unassigned: Option<crate::v5_0::types::EaDetailsUnassignedType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EaEmitterDataTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EmitterType: Option<crate::v5_0::types::EmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalDescription: Option<crate::v5_0::types::SignalSummaryType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalId: Option<crate::v5_0::types::SignalIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EaPowerTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PowerAtTarget: Option<crate::v5_0::common::DecibelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Erp: Option<crate::v5_0::common::DecibelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub JtoS: Option<crate::v5_0::common::DecibelType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EaProposedTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActivityId: Option<crate::v5_0::types::ActivityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Entity: Option<crate::v5_0::types::EaEntityType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EaTargetPointingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LocationData: Option<crate::v5_0::choices::TargetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirVolume: Option<crate::v5_0::types::AirVolumeSensorReferencedType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EaTaskRouteRequirementsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Path: Option<crate::v5_0::types::PathType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Loiter: Option<crate::v5_0::choices::LoiterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Escort: Option<crate::v5_0::types::EaTaskEscortType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ZoneConstraints: Option<crate::v5_0::choices::ZoneChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VolumeConstraints: Option<crate::v5_0::choices::VolumeChoiceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EaTaskThreatsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SuppressAll: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SuppressionConstraints: Option<Vec<crate::v5_0::types::EaTaskSuppressionConstraintsType>>,
}

#[doc = r#"Defines the type that allows a choice of Earth Orientation Parameters data type: EarthOrientatonParameters message or static values."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EarthOrientationParametersDataChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EarthOrientationParametersId: Option<crate::v5_0::types::EarthOrientationParametersIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StaticValues: Option<crate::v5_0::types::TimeAndPolarDataType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EffectCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::EffectCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::ActivityCommandBaseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EffectPlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EffectPlanCommandId: Option<crate::v5_0::types::EffectPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EffectPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EmconErpTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MaximumRadiatedErp: Option<crate::v5_0::common::MilliwattPowerRatioType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RadiateFullPower: Option<bool>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EmconOverrideTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EmconLevel: Option<crate::v5_0::enums::EmconLevelEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ForeignLevel: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EmconSettingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EmconLevel: Option<crate::v5_0::enums::EmconLevelEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ForeignLevel: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"Container object for the different types of OpPoint*Enums, excluding Emergency.  A separate enum applies to each of the choice types."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EmergencyReferenceOpPointCategoriesTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub General: Option<crate::v5_0::enums::OpPointGeneralEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Hazard: Option<crate::v5_0::enums::OpPointHazardEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Reference: Option<crate::v5_0::enums::OpPointReferenceEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Station: Option<crate::v5_0::enums::OpPointStationEnum>,
}

#[doc = r#"Specify an emitter by ID or by MDF_Entry number."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EmitterEntryTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Emitter: Option<crate::v5_0::types::EmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MdfEntry: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"Indicates the emitter identification based on its category."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EmitterIdentityCategoryTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Radar: Option<crate::v5_0::types::RadarEmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Communications: Option<crate::v5_0::types::CommunicationsEmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Jammer: Option<crate::v5_0::types::JammerEmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Missile: Option<crate::v5_0::types::MissileEmitterIdentityType>,
}

#[doc = r#"Entity ID or Local Track ID of the emitter used to detect targets passively."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EmitterSourceIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OmrIndividualMeasurementId: Option<crate::v5_0::types::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EobEmitterId: Option<crate::v5_0::types::EobEmitterIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalId: Option<crate::v5_0::types::SignalIdType>,
}

#[doc = r#"Source emitter location. Used if Waveform does not contain location."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EmitterTargetLocationDataTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EmitterSourceId: Option<crate::v5_0::choices::EmitterSourceIdChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Fov: Option<crate::v5_0::types::FovVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PointTarget: Option<crate::v5_0::types::PointTargetType>,
}

#[doc = r#"An ellipse or rectangle shape describing 1-sigma position uncertainty."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EmitterUncertaintyChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UncertaintyEllipse: Option<crate::v5_0::types::EllipseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UncertaintyRectangle: Option<crate::v5_0::types::RectangleType>,
}

#[doc = r#"Indicates the source of or explicit values for emitter characteristics of the emitter used as a source for passive detection."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EmitterWaveformDataTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EmitterType: Option<crate::v5_0::types::EmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificEmitter: Option<crate::v5_0::types::SpecificEmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalDescription: Option<crate::v5_0::types::SignalParametricsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalId: Option<crate::v5_0::types::SignalIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EobEmitterModeId: Option<crate::v5_0::types::EobEmitterModeIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EndPointTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WayPoint: Option<crate::v5_0::types::WayPointType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TurnPoint: Option<crate::v5_0::types::TurnPointType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LoiterPoint: Option<crate::v5_0::types::LoiterPointType>,
}

#[doc = r#"Indicates endurance in terms of the domain specific choice."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EnduranceMultiStandardTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EnduranceFootprint: Option<crate::v5_0::types::EnduranceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SatelliteEndurance: Option<crate::v5_0::types::SatelliteEnduranceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EntityCharacteristicTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Identity: Option<crate::v5_0::types::IdentityComparisonType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub IdentityStaleness: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PositionUncertainty: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub PositionStaleness: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PrioritizationList: Option<crate::v5_0::types::PrioritizationListValueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Behavior: Option<crate::v5_0::types::BehaviorType>,
}

#[doc = r#"Indicates the contributors to the fused entity.  This type allows specifying non-Entity contributors if a fusion service supports this functionality."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EntityContributorIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EobEmitterId: Option<crate::v5_0::types::EobEmitterIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalId: Option<crate::v5_0::types::SignalIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SobC2RecordId: Option<crate::v5_0::types::SobC2RecordIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SobSatelliteRecordId: Option<crate::v5_0::types::SobSatelliteRecordIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MeasurementId: Option<crate::v5_0::types::MeasurementIdType>,
}

#[doc = r#"A choice type that determines whether the ElementSet, EntityElementSetID, or KinematicVector will be used to determine the ephemeris for the Entity or Entities."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EntityEphemerisBasisChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ElementSet: Option<crate::v5_0::types::TleBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityElementSetId: Option<crate::v5_0::types::EntityOrbitalElementSetIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub KinematicVector: Option<crate::v5_0::choices::OrbitalKinematicsStandardFrameChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityVcmId: Option<crate::v5_0::types::EntityOrbitalVcmIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EntityIdentityChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Standard: Option<crate::v5_0::types::StandardIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Environment: Option<crate::v5_0::types::EnvironmentIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Platform: Option<crate::v5_0::types::PlatformIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Specific: Option<crate::v5_0::types::SpecificIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Emitter: Option<crate::v5_0::types::EmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificEmitter: Option<crate::v5_0::types::SpecificEmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificVehicle: Option<crate::v5_0::types::VehicleIdentificationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificFacility: Option<crate::v5_0::choices::FacilityIdentificationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Eob: Option<crate::v5_0::choices::EobIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Weapon: Option<crate::v5_0::types::StoreType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EntityManagementDropTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<Vec<crate::v5_0::types::EntityIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DropPolicy: Option<crate::v5_0::enums::EntityDropPolicyEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EntityManagementRequestTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Split: Option<crate::v5_0::types::EntityManagementSplitType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Merge: Option<crate::v5_0::types::EntityManagementMergeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Drop: Option<crate::v5_0::choices::EntityManagementDropType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetDropRestriction: Option<crate::v5_0::types::EntityManagementDropRestrictType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ClearDropRestriction: Option<crate::v5_0::types::EntityManagementDropRestrictType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Swap: Option<crate::v5_0::types::EntityManagementSwapType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetFusionEligibility: Option<crate::v5_0::types::EntityManagementSetFusionEligibilityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetIdentity: Option<crate::v5_0::types::EntityManagementSetIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ClearIff: Option<crate::v5_0::types::EntityManagementClearIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetDownLocation: Option<crate::v5_0::types::EntityManagementDownType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetKinematics: Option<crate::v5_0::types::EntityManagementKinematicsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetMode: Option<crate::v5_0::types::EntityManagementSetModeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetActivityBy: Option<crate::v5_0::types::EntityManagementSetActivityByType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetStrength: Option<crate::v5_0::types::EntityManagementSetStrengthType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetPlatformStatus: Option<crate::v5_0::types::EntityManagementSetPlatformStatusType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetEndurance: Option<crate::v5_0::types::EntityManagementSetEnduranceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetLink16Metadata: Option<crate::v5_0::types::EntityManagementSetLink16MetadataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProposeCorrelation: Option<crate::v5_0::types::EntityManagementProposeCorrelationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetVoiceControl: Option<crate::v5_0::types::EntityManagementSetVoiceControlType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetCapability: Option<crate::v5_0::types::EntityManagementSetCapabilityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetSignalSummary: Option<crate::v5_0::types::EntityManagementSetSignalSummaryType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EntitySourceIdentifierTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EobRecordId: Option<crate::v5_0::types::EobRecordIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ExternalIdentifier: Option<crate::v5_0::types::EntityExternalType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Fusion: Option<Vec<crate::v5_0::types::EntityFusionSourceType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InternallyDerivedId: Option<crate::v5_0::types::IdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::EntityCapabilitySourceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductMetadataId: Option<Vec<crate::v5_0::types::ProductMetadataIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OperatorId: Option<crate::v5_0::types::OperatorIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SobSatelliteRecordId: Option<crate::v5_0::types::SobSatelliteRecordIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SobC2RecordId: Option<crate::v5_0::types::SobC2RecordIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EobIdentityTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Site: Option<crate::v5_0::types::EobSiteIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Equipment: Option<crate::v5_0::types::EobEquipmentIdentityType>,
}

#[doc = r#"Indicates a choice between propagation parameters. Allows either the selection of USSF Astrodynamic Standards orbital model parameters or more general propagator settings."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EphemerisPropagatorChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalModelPropagator: Option<crate::v5_0::types::EphemerisOrbitalModelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PropagatorSettings: Option<crate::v5_0::choices::PropagatorSettingsChoiceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EsmAcquisitionTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EmitterType: Option<crate::v5_0::types::EmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificEmitter: Option<crate::v5_0::types::SpecificEmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalDescription: Option<crate::v5_0::types::SignalSummaryType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalId: Option<crate::v5_0::types::SignalIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EsmCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::EsmCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::EsmActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EsmLocationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TargetLocationData: Option<crate::v5_0::choices::EsmSubcapabilityTargetLocationDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EsmAirVolume: Option<crate::v5_0::types::AirVolumeSensorReferencedType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EsmSubcapabilityTargetLocationDataTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DwellFov: Option<crate::v5_0::types::NedConeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PointTarget: Option<crate::v5_0::types::PointTargetType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EsmTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EmitterType: Option<crate::v5_0::types::EmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificEmitter: Option<crate::v5_0::types::SpecificEmitterIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalDescription: Option<crate::v5_0::types::SignalSummaryType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalId: Option<crate::v5_0::types::SignalIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EmitterPriorityBin: Option<u32>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EthernetSettingsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ipv4: Option<crate::v5_0::types::Ipv4SettingsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ipv6: Option<crate::v5_0::types::Ipv6SettingsType>,
}

#[doc = r#"Provides a choice of event offset types."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EventOffsetChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub OffsetTime: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OffsetAngle: Option<crate::v5_0::common::AngleType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AzEl: Option<crate::v5_0::types::LosInertialBType>,
}

#[doc = r#"Provides a choice of event window size definitions."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct EventWindowChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WindowAngle: Option<crate::v5_0::common::AnglePositiveType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub WindowDuration: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WindowRadius: Option<crate::v5_0::common::DistanceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ExecutionSequenceInsertionTypeChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InsertAtStart: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InsertAfterExecutionPlanSetId: Option<crate::v5_0::types::ExecutionPlanSetIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ExecutionSequenceReplaceOrModifyChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReplaceExecutionSequence: Option<crate::v5_0::types::ExecutionSequenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ModifyExecutionSequence: Option<crate::v5_0::types::ExecutionSequenceModificationDetailsType>,
}

#[doc = r#"Provides identification of an object associated with the Air Force Space Command."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct FacilityIdentificationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SiteIdentifier: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SensorIdentifier: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ForeignFacilityKey: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"Encoding types for CVEnumISMCATFGIOpen Version 2 controlled vocabulary enumerations.  Derived from the CVEnumISMCATFGIOpen.xml CVE.(U) 
				  All currently valid GENC trigraphs except USA in alphabetical order by trigraph, 
				  followed by all currently valid CAPCO Coalition tetragraphs in alphabetical order by tetragraph. UNKNOWN removed since GENC has it as AX1

						   PERMISSIBLE VALUES

						   The permissible values for this simple type are defined in the Controlled Value Enumeration:

						   CVEnumISMCATFGIOpen.xml"#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct FgiSourceOpenChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ForeignGovernmentIdentifier: Option<crate::v5_0::enums::FgiSourceOpenEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NatoSpecialWord: Option<crate::v5_0::common::NatoSpecialWordsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct FileFormatTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Mime: Option<crate::v5_0::common::MimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NonMime: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct FusionSettingsRequestTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetStalenessThresholds: Option<crate::v5_0::types::EntityStalenessThresholdsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetMergeAndDropSettings: Option<crate::v5_0::types::FusionSettingsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetInputSources: Option<crate::v5_0::types::FusionSourcesType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct FuzeTriggerTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FuzeDistance: Option<crate::v5_0::common::DistanceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub FuzeDelayTime: Option<chrono::TimeDelta>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct GatewayCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::GatewayCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::GatewayActivityCommandType>,
}

#[doc = r#"Contains methods for describing geographic area characteristics of a Link 16 filter."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct GatewayLink16ConfigurationAreaTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Circle: Option<crate::v5_0::types::GatewayLink16ConfigurationCircleType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Annulus: Option<crate::v5_0::types::GatewayLink16ConfigurationAnnulusType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CircularSector: Option<crate::v5_0::types::GatewayLink16ConfigurationCircularSectorType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Rectangle: Option<crate::v5_0::types::GatewayLink16ConfigurationRectangleType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Line: Option<crate::v5_0::types::GatewayLink16ConfigurationLineType>,
}

#[doc = r#"Contains methods for describing movement characteristics of a Link 16 filter."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct GatewayLink16ConfigurationMotionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Moving: Option<crate::v5_0::types::GatewayLink16ConfigurationMovementVectorType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Slaved: Option<crate::v5_0::choices::GatewayLink16ConfigurationSlaveType>,
}

#[doc = r#"Identifies a geographically-located item to which a Link 16 filter is slaved, meaning that the filter's current location should be considered to always be relative to the location of the identified item."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct GatewayLink16ConfigurationSlaveTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OperatorLocationOfInterestId: Option<crate::v5_0::types::OperatorLocationOfInterestIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPointId: Option<crate::v5_0::types::OpPointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
}

#[doc = r#"Container to reference the appropriate geo-located object."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct GeoLocatedObjectTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPointId: Option<crate::v5_0::types::OpPointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpLineId: Option<crate::v5_0::types::OpLineIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZoneId: Option<crate::v5_0::types::OpZoneIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolumeId: Option<crate::v5_0::types::OpVolumeIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DmpiId: Option<crate::v5_0::types::DmpiIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalReportId: Option<crate::v5_0::types::SignalReportIdType>,
}

#[doc = r#"A choice between archived objects with a defined location."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct GeoLocatedStoredObjectTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Entity: Option<crate::v5_0::types::EntityDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub System: Option<crate::v5_0::types::SystemDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPoint: Option<crate::v5_0::types::EmergencyReferenceOpPointType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpLine: Option<crate::v5_0::types::OpLineMdt>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZone: Option<crate::v5_0::types::OpZoneMdt>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolume: Option<crate::v5_0::types::OpVolumeMdt>,
}

#[doc = r#"Provides the container that allows for specifying ways to identify the battlespace object."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct IdentityKindAssetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByInstance: Option<crate::v5_0::choices::AssetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByIdentity: Option<crate::v5_0::types::IdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByPlan: Option<crate::v5_0::types::RequirementPlanningCandidateType>,
}

#[doc = r#"Provides the container that allows for specifying ways to identify the battlespace object."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct IdentityKindInstanceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByInstance: Option<crate::v5_0::choices::TargetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByIdentity: Option<crate::v5_0::types::IdentityType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct IffActiveModesTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IffTransponderModes: Option<crate::v5_0::types::IffActivityTransponderType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IffInterrogationModes: Option<crate::v5_0::types::IffModeSelectionType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct IffCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::IffCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::IffActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct IffInterrogatorTransponderModesTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TransponderModesControl: Option<crate::v5_0::types::IffTransponderModeControlType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InterrogatorModesEnable: Option<crate::v5_0::types::IffInterrogatorModesEnableType>,
}

#[doc = r#"NITF ImageSubheader Image Identifier 2, defined herein for either non-IPON-compliant NTIF producers or IPON-compliant NITF producers, mutually exclusively."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Iid2ChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NonIponIid2: Option<crate::v5_0::common::NonIponIid2Type>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IponIid2: Option<crate::v5_0::types::IponIid2Type>,
}

#[doc = r#"Indicates the point of impact."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ImpactPointTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BodyLocation: Option<crate::v5_0::types::OffsetLocationErrorType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BodyFace: Option<crate::v5_0::types::BodyFaceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct InputProductTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductReference: Option<crate::v5_0::choices::ProductReferenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductTypeReference: Option<Vec<crate::v5_0::choices::ProductFilterType>>,
}

#[doc = r#"The type used to specify a distance or duration type for an interval."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct IntervalChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Distance: Option<crate::v5_0::common::DistanceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub Duration: Option<chrono::TimeDelta>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct IoPortConfigurationStatusTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EthernetSettings: Option<crate::v5_0::choices::EthernetSettingsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SerialPortSettings: Option<crate::v5_0::types::CommPortSettingsType>,
}

#[doc = r#"Specifies an IPv4 or IPv6 connection."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct IpConnectionChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ipv4: Option<crate::v5_0::types::Ipv4ConnectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ipv6: Option<crate::v5_0::types::Ipv6ConnectionType>,
}

#[doc = r#"Specifies a server, client, or multicast IP connection."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct IpConnectionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Server: Option<crate::v5_0::choices::IpConnectionChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Client: Option<crate::v5_0::choices::IpConnectionChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Multicast: Option<crate::v5_0::choices::IpConnectionChoiceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct IsarTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RawTarget: Option<crate::v5_0::types::PointTargetType>,
}

#[doc = r#"Indicates the kinematics expressed in one of several different kinematics standards."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct KinematicsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VolumeKinematics: Option<crate::v5_0::types::OpVolumeKinematicsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalKinematics: Option<crate::v5_0::choices::OrbitalKinematicsChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LocalBodyPosition: Option<crate::v5_0::types::RtnLocalPositionType>,
}

#[doc = r#"Provides a choice of ways to express kinematics in one of several orbital kinematics frames/standards."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct KinematicsMultiStandardTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Orbital: Option<crate::v5_0::choices::OrbitalKinematicsChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Wgs: Option<crate::v5_0::types::KinematicsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DeltaOrbitalPlaneTolerance: Option<crate::v5_0::types::AngleHalfPairType>,
}

#[doc = r#"Option to implicitly or explicitly provide the kinematics of an Entity or System.

This allows the option to override kinematics information for a known system/entity when there is no known kinematics information or the information is not appropriate (e.g. outdated) by the time of use."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct KinematicsOptionsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReferenceAsset: Option<crate::v5_0::choices::AssetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub KinematicsOverride: Option<crate::v5_0::choices::KinematicsMultiStandardType>,
}

#[doc = r#"Provides a choice of line of sight vector definitions."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct LineOfSightChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LosAzEl: Option<crate::v5_0::types::LosMeasurementAndUncertaintyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Los3dKinematics: Option<crate::v5_0::types::Los3dKinematicsType>,
}

#[doc = r#"Choice of either relative or geospatial point representing the vertex of a line."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct LinePointChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Point: Option<Vec<crate::v5_0::types::LinePoint2DType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RelativePoint: Option<crate::v5_0::types::LineRelativeType>,
}

#[doc = r#"Stores the ID of an EW."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Link16EwDataStoreIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<Vec<crate::v5_0::types::EntityIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalReportId: Option<Vec<crate::v5_0::types::SignalReportIdType>>,
}

#[doc = r#"Stores the ID of a Friendly Target of Interest in a Link16 setting."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Link16FriendlyTargetofInterestDataStoreIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<Vec<crate::v5_0::types::EntityIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpLineId: Option<Vec<crate::v5_0::types::OpLineIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPointId: Option<Vec<crate::v5_0::types::OpPointIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZoneId: Option<Vec<crate::v5_0::types::OpZoneIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolumeId: Option<Vec<crate::v5_0::types::OpVolumeIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<Vec<crate::v5_0::types::SystemIdType>>,
}

#[doc = r#"Provides information about the OpZone, OpVolume, OpLine, and OpPoint ID."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Link16ReferencePointDataStoreIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpLineId: Option<Vec<crate::v5_0::types::OpLineIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPointId: Option<Vec<crate::v5_0::types::OpPointIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZoneId: Option<Vec<crate::v5_0::types::OpZoneIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolumeId: Option<Vec<crate::v5_0::types::OpVolumeIdType>>,
}

#[doc = r#"Specifies that the content being filtered must be in the specified zone if the zone is marked inclusionary or outside of the zone if the zone is marked exclusionary."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct LocationFilterTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Zone: Option<crate::v5_0::types::ZoneInclusionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Location: Option<crate::v5_0::types::Point2DType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct LocationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Path: Option<crate::v5_0::types::PathType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Loiter: Option<crate::v5_0::choices::LoiterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Zone: Option<crate::v5_0::types::ZoneType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct LoiterProgressTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LoiterEndTime: Option<chrono::DateTime<chrono::Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CompletedOrbits: Option<u32>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct LoiterTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Orbit: Option<crate::v5_0::types::OrbitType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Hover: Option<crate::v5_0::types::HoverType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct LosDTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Los: Option<crate::v5_0::types::LosVariableBType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LosRates: Option<crate::v5_0::types::LosRatesType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct LostLinkSourceIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityId: Option<crate::v5_0::types::CapabilityIdType>,
}

#[doc = r#"Provides a choice between body reference orientation rates and unitless Mach value for the specification of a paired value to an aircraft acceleration limit."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaAccelerationLimitPairTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BodyReferenceOrientationRate: Option<crate::v5_0::types::MaBodyReferenceOrientationRateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MachValue: Option<crate::v5_0::common::MachType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaActionCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::MaActionCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::ActivityCommandBaseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaAllocationChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<Vec<crate::v5_0::types::SystemIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PackageId: Option<crate::v5_0::types::PackageIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaApprovalRequestItemReferenceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlanApproval: Option<crate::v5_0::choices::PlanReferenceIdChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementExecutionApproval: Option<crate::v5_0::types::ApprovalRequestItemType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanActivationApproval: Option<Vec<crate::v5_0::types::MaMissionPlanActivationCommandType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaAssessmentRequestTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommPointingPlan: Option<crate::v5_0::types::CommPointingPlanRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityUtilization: Option<crate::v5_0::types::CapabilityUtilizationRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteDeconfliction: Option<crate::v5_0::types::RouteDeconflictionRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteVulnerabilityMetrics: Option<crate::v5_0::types::RouteVulnerabilityMetricsRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteThreatAssessment: Option<crate::v5_0::types::ThreatAssessmentRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TargetMobility: Option<crate::v5_0::types::TargetMobilityRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VehicleThreatAssessment: Option<crate::v5_0::types::VehicleThreatAssessmentRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ThreatNominationAssessment: Option<crate::v5_0::types::ThreatNominationAssessmentRequestType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AchievabilityAssessment: Option<crate::v5_0::types::AchievabilityAssessmentRequestPet>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CoverageAssessment: Option<crate::v5_0::types::MaCoverageAssessmentRequestType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaAssessmentTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommPointingPlan: Option<crate::v5_0::types::CommPointingPlanAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityUtilization: Option<crate::v5_0::types::CapabilityUtilizationAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteDeconfliction: Option<crate::v5_0::types::RouteDeconflictionAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteVulnerabilityMetrics: Option<crate::v5_0::types::RouteVulnerabilityMetricsAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteThreatAssessment: Option<crate::v5_0::types::RouteThreatAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TargetMobility: Option<crate::v5_0::types::TargetMobilityAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VehicleThreatAssessment: Option<crate::v5_0::types::VehicleThreatAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ThreatNominationAssessment: Option<crate::v5_0::types::ThreatNominationAssessmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AchievabilityAssessment: Option<crate::v5_0::types::AchievabilityAssessmentPet>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CoverageAssessment: Option<crate::v5_0::types::MaCoverageAssessmentType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaAuthorizedZoneChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AuthorizedZone: Option<crate::v5_0::types::ZoneType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AuthorizedOpZoneId: Option<crate::v5_0::types::OpZoneIdType>,
}

#[doc = r#"Defines the type and value of the offset (delay relative to the reference system) used for CAP synchronization."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaCapoffsetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Time: Option<crate::v5_0::types::DurationRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Distance: Option<crate::v5_0::common::DistanceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PercentofPattern: Option<crate::v5_0::common::PercentType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaCarrierRecoveryChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Recovery: Option<crate::v5_0::types::MaCarrierRecoveryType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Delta: Option<crate::v5_0::types::MaDeltaType>,
}

#[doc = r#"The Command to be applied to a node or a system. The command can be either a String or hexBinary."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaCommandValueTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub HexBinaryValue: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StringValue: Option<crate::v5_0::common::VisibleString256Type>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaConstraintPriorityTypeChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ZoneCategory: Option<crate::v5_0::enums::MaOpZoneCategoryEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VolumeCategory: Option<crate::v5_0::enums::MaOpZoneCategoryEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LineCategory: Option<crate::v5_0::enums::OpLineCategoryEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaControlChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GrantedControlType: Option<crate::v5_0::enums::MaControlTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PermittedControlType: Option<crate::v5_0::enums::MaPermittedControlTypeEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaCopConfigurationParticipantTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CopRole: Option<Vec<crate::v5_0::enums::MaCopConfigurationRoleEnum>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<Vec<crate::v5_0::types::SystemIdType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaCopConfigurationTimingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Periodic: Option<crate::v5_0::types::MaPeriodRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ChangeThreshold: Option<crate::v5_0::common::PercentType>,
}

#[doc = r#"Contains speed or time contraints to applied for a curve following command"#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaCurveTraversingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpeedRange: Option<crate::v5_0::types::SpeedRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Duration: Option<i64>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaDataActivationAuthorizationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AuthorizedExceptFor: Option<crate::v5_0::types::MaDataActivationPermissionsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnauthorizedExceptFor: Option<crate::v5_0::types::MaDataActivationPermissionsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaDataModificationAuthorizationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AuthorizedExceptFor: Option<crate::v5_0::types::MaDataModificationPermissionsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnauthorizedExceptFor: Option<crate::v5_0::types::MaDataModificationPermissionsType>,
}

#[doc = r#"Enables the specification of a heading or course type with an associated reference."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaDirectionChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Heading: Option<crate::v5_0::types::MaDirectionReferenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Course: Option<crate::v5_0::types::MaDirectionReferenceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaDynamicFitnessScoreChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub C2CommsBinary: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub P2pCommsCount: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaElectromagneticIndicationChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Iff: Option<crate::v5_0::types::IffType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificEmitterKey: Option<crate::v5_0::types::ForeignKeyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CenotIdentifier: Option<crate::v5_0::common::NotationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ElnotIdentifier: Option<crate::v5_0::common::NotationType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaEndPointTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WayPoint: Option<crate::v5_0::types::WayPointType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TurnPoint: Option<crate::v5_0::types::TurnPointType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LoiterPoint: Option<crate::v5_0::types::MaLoiterPointType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaEntityCharacteristicTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Identity: Option<crate::v5_0::types::IdentityComparisonType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub IdentityStaleness: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PositionUncertainty: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub PositionStaleness: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PrioritizationList: Option<crate::v5_0::types::MaPrioritizationListValueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Behavior: Option<crate::v5_0::types::BehaviorType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaEscortAssetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaEscortCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::MaEscortCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::MaEscortActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaEscortReferenceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FollowPath: Option<crate::v5_0::types::MaEscortFollowPathType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeographicReference: Option<crate::v5_0::types::LosType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BodyReference: Option<crate::v5_0::types::LosType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaExecutionPriorityTypeChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TaskType: Option<crate::v5_0::enums::MaTaskTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActionType: Option<crate::v5_0::enums::ActionTypeEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaFlightCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::MaFlightCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::MaFlightActivityCommandType>,
}

#[doc = r#"Provides a choice among available flight control mode types."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaFlightControlModesChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub HsaCsa: Option<crate::v5_0::types::MaHsaCsaType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WaypointFollowing: Option<crate::v5_0::types::MaWaypointFollowingType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CurveFollowing: Option<crate::v5_0::types::MaCurveControlType>,
}

#[doc = r#"Defines details of orbit loiter"#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaHoldLegSpecificationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub LegTime: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LegLength: Option<crate::v5_0::common::DistanceType>,
}

#[doc = r#"Specifies the turns of the hold."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaHoldTurnSpecificationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TurnRadius: Option<crate::v5_0::common::DistanceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TurnRate: Option<crate::v5_0::common::AngleRateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TurnType: Option<crate::v5_0::enums::MaHoldTurnTypeEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaInterceptTacticTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SkateType: Option<crate::v5_0::types::MaSkateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BanzaiType: Option<crate::v5_0::types::MaBanzaiType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaJettisonStoreSelectionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub JettisonAllCapableWeapons: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityId: Option<Vec<crate::v5_0::types::CapabilityIdType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaLaunchTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CarrierLaunch: Option<crate::v5_0::types::MaCarrierLaunchType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirfieldTakeoff: Option<crate::v5_0::types::MaAirfieldTakeoffType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaLeadershipFitnessScoreDeterminationMethodChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StaticFitnessScore: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UserDefined: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DynamicFitnessScore: Option<crate::v5_0::choices::MaDynamicFitnessScoreChoiceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaLoiterTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Orbit: Option<crate::v5_0::types::MaOrbitType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Hover: Option<crate::v5_0::types::HoverType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Hold: Option<crate::v5_0::types::MaHoldType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaMissionEnvironmentConstraintTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedEntity: Option<crate::v5_0::types::ConstrainedEntityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedOpPoint: Option<crate::v5_0::types::ConstrainedOpPointType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedOpLine: Option<crate::v5_0::types::ConstrainedOpLineType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedOpZone: Option<crate::v5_0::types::ConstrainedOpZoneType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedOpVolume: Option<crate::v5_0::types::ConstrainedOpVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub System: Option<crate::v5_0::types::SystemStatusMdt>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RiskAdjustment: Option<crate::v5_0::types::MaRequirementRiskAdjustmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Parameter: Option<crate::v5_0::types::ParameterAssertType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaMissionPlanActivationDetailsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByMissionPlan: Option<crate::v5_0::types::MissionPlanActivationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BySubPlan: Option<crate::v5_0::types::MaMissionPlanSubplanActivationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByExecutionPlanSet: Option<crate::v5_0::types::MaExecutionPlanSetActivationType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaMissionPlanningAutonomyResponseChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutonomousPlanningAction: Option<Vec<crate::v5_0::types::MaPlanningAllowedEscalationType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AlertOnly: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaMissionPlanningByResultAutonomousActionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlanningAllowed: Option<Vec<crate::v5_0::types::MaPlanningAllowedType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AlertOnly: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"The choice of a package or task to describe the leader update message."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaNominationChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ApplicableTaskId: Option<crate::v5_0::types::TaskIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ApplicablePackageId: Option<crate::v5_0::types::PackageIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaOpPlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"Contains volume location and geometry information."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaOpVolumeChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeometrySpecification: Option<crate::v5_0::choices::OpVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeometryReference: Option<crate::v5_0::types::MaReferentialOpVolumeType>,
}

#[doc = r#"Container for parameters that are unique to a specific enumeration in OpZoneCategoryEnum.  For example, KeepIn enumeration can have amplifying information such as entry and exit restrictions of the zone."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaOpZoneCategoryTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedEntryExit: Option<crate::v5_0::types::ConstrainedEntryExitType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FilterArea: Option<Vec<crate::v5_0::types::OpZoneFilterAreaPet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Jamming: Option<crate::v5_0::types::OpZoneJammingType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub KeepIn: Option<crate::v5_0::types::IngressEgressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissileLaunchPoint: Option<crate::v5_0::types::OpZoneMissileDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NoFire: Option<crate::v5_0::types::OpZoneNoFireType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NoFly: Option<crate::v5_0::types::OpZoneNoFlyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpeedLimits: Option<crate::v5_0::types::MaOpZoneSpeedLimitsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VehicleConfiguration: Option<crate::v5_0::types::MaVehicleCommandDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeaponRestriction: Option<crate::v5_0::types::OpZoneWeaponRestrictionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeatherConditions: Option<crate::v5_0::types::OpZoneWeatherType>,
}

#[doc = r#"Contains zone location and geometry information."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaOpZoneChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeometrySpecification: Option<crate::v5_0::types::OpZoneType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeometryReference: Option<crate::v5_0::types::MaReferentialOpZoneType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaOperatorRequestAuthorizationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AuthorizedExceptFor: Option<crate::v5_0::types::MaOperatorRequestPermissionsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnauthorizedExceptFor: Option<crate::v5_0::types::MaOperatorRequestPermissionsType>,
}

#[doc = r#"Specifies the turns of the hold."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaOrbitDurationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub Time: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NumberOfOrbits: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntryExitTime: Option<crate::v5_0::types::DateTimeRangeType>,
}

#[doc = r#"Defines shape of orbit"#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaOrbitShapeTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FixPoint: Option<crate::v5_0::types::MaFixOrbitType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Circle: Option<crate::v5_0::types::MaCircleOrbitType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaPackageSystemTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub All: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Non: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PackageId: Option<crate::v5_0::types::PackageIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaPlanAuthorizationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AuthorizedExceptFor: Option<crate::v5_0::types::MaPlanActivationPermissionsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnauthorizedExceptFor: Option<crate::v5_0::types::MaPlanActivationPermissionsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaPlanningByCaseTriggerTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityAdded: Option<crate::v5_0::types::CapabilityTaxonomyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityFailure: Option<crate::v5_0::types::CapabilityTaxonomyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommsLost: Option<crate::v5_0::types::CommsLostTriggerDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DmpiOverDesignation: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DmpiUnderDesignation: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EnduranceLow: Option<crate::v5_0::types::EnduranceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OffRoute: Option<crate::v5_0::types::ThresholdOffRouteTriggerDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProximityConflict: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReleasePointOutsideLar: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteConflict: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteVulnerability: Option<crate::v5_0::types::PlanVulnerabilityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemStateChange: Option<crate::v5_0::types::SystemStateFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementAdded: Option<crate::v5_0::types::MaRequirementTriggerType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementDependencyFailed: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementDropped: Option<crate::v5_0::types::MaRequirementTriggerType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementFailed: Option<crate::v5_0::types::MaRequirementFailedTriggerType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementChange: Option<crate::v5_0::types::MaRequirementTriggerType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementTiming: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ZoneViolation: Option<crate::v5_0::types::ZoneViolationTriggerDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitConflict: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OffPlannedOrbit: Option<crate::v5_0::types::ThresholdOffOrbitTriggerDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpacecraftEnduranceLow: Option<crate::v5_0::types::SatelliteEnduranceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpacecraftProximityConflict: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ResponseId: Option<crate::v5_0::types::ResponseIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaPlanningByResultTriggerTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReplanRequired: Option<Vec<crate::v5_0::enums::PlanTypeEnum>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VulnerabilityChanged: Option<crate::v5_0::types::PlanVulnerabilityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementUnallocated: Option<crate::v5_0::types::MaRequirementTriggerType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaRecoveryTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CarrierRecovery: Option<crate::v5_0::choices::MaCarrierRecoveryChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirfieldLand: Option<crate::v5_0::types::MaAirfieldLandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaRegionChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZoneId: Option<crate::v5_0::types::OpZoneIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolumeId: Option<crate::v5_0::types::OpVolumeIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Zone: Option<crate::v5_0::types::ZoneType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaRequirementAuthorizationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AuthorizedExceptFor: Option<crate::v5_0::types::MaRequirementAuthorizationPermissionsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnauthorizedExceptFor: Option<crate::v5_0::types::MaRequirementAuthorizationPermissionsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaRequirementChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByType: Option<crate::v5_0::choices::MaRequirementTaxonomyChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByInstance: Option<crate::v5_0::choices::RequirementInstanceIdChoiceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaRequirementTaxonomyChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Effect: Option<crate::v5_0::enums::EffectTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Action: Option<crate::v5_0::enums::ActionTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Task: Option<crate::v5_0::enums::MaTaskTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityCommand: Option<crate::v5_0::enums::CapabilityTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Response: Option<crate::v5_0::enums::ResponseTypeEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaRuleResponseTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementsTemplate: Option<crate::v5_0::types::ResponseTemplateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActivatePlan: Option<crate::v5_0::types::MaMissionPlanActivationCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GenerateAlert: Option<crate::v5_0::types::ResponseAlertType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DoNothing: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaSettingsModficationAuthorizationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AuthorizedExceptFor: Option<crate::v5_0::types::MaPlatformSettingsPermissionsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnauthorizedExceptFor: Option<crate::v5_0::types::MaPlatformSettingsPermissionsType>,
}

#[doc = r#"Represents the type of subscription and optional filters for the subscription."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaSubscriptionFilterTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UciMessage: Option<crate::v5_0::types::QueryMessageType>,
}

#[doc = r#"Defines the CAP synchronization method to be utilized."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaSynchronizationChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Strategy: Option<crate::v5_0::enums::MaCapsynchronizationTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RelativePositioning: Option<Vec<crate::v5_0::types::MaCaprelativePositionType>>,
}

#[doc = r#"Provides choice to provide a known System ID or a description of a set of capabilities."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaSystemCandidateChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemCandidate: Option<Vec<crate::v5_0::types::SystemCandidateType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemCapabilityDescription: Option<crate::v5_0::types::MaCapabilityTaxonomyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaSystemCharacteristicTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Identity: Option<crate::v5_0::choices::EntityIdentityChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PositionUncertainty: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub PositionStaleness: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PrioritizationList: Option<crate::v5_0::types::MaPrioritizationListValueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Behavior: Option<crate::v5_0::types::BehaviorType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaSystemManagementRequestTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetMode: Option<crate::v5_0::enums::MessageModeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetIdentity: Option<crate::v5_0::types::SystemIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetLink16Metadata: Option<crate::v5_0::types::Link16MetadataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetVoiceControl: Option<crate::v5_0::types::VoiceControlType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetSensorEntityReporting: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VehicleSettings: Option<crate::v5_0::types::MaVehicleCommandDataType>,
}

#[doc = r#"Identifies the type of this Task instance. Note: When modifying this complexType (whether adding or removing choices), there are equivalent complexTypes that require the same modifications. Changes to this type may necessitate a modification to CapabilityTaxonomyType."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaTaskTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirSample: Option<crate::v5_0::types::AirSampleTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Amti: Option<crate::v5_0::types::AmtiTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ao: Option<crate::v5_0::types::AoTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Cap: Option<crate::v5_0::types::MaCapTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CargoDelivery: Option<crate::v5_0::choices::CargoDeliveryTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Comint: Option<crate::v5_0::types::ComintTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommRelay: Option<crate::v5_0::types::CommRelayTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CounterSpace: Option<crate::v5_0::types::CounterSpaceTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Escort: Option<crate::v5_0::types::MaEscortTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ea: Option<crate::v5_0::types::EaTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Esm: Option<crate::v5_0::types::EsmTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Flight: Option<crate::v5_0::types::MaFlightTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Jettison: Option<crate::v5_0::types::MaJettisonTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitChange: Option<crate::v5_0::types::OrbitChangeTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalSurveillance: Option<crate::v5_0::types::OrbitalSurveillanceTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalSurveillanceSensor: Option<crate::v5_0::types::OrbitalSurveillanceSensorTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Po: Option<crate::v5_0::types::PoTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Refuel: Option<crate::v5_0::types::RefuelTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Sar: Option<crate::v5_0::types::SarTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Smti: Option<crate::v5_0::types::SmtiTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Strike: Option<crate::v5_0::types::StrikeTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemDeployment: Option<crate::v5_0::types::SystemDeploymentTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TacticalOrder: Option<crate::v5_0::types::TacticalOrderTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeatherRadar: Option<crate::v5_0::types::WeatherRadarTaskType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaUserTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OperatorRoleId: Option<crate::v5_0::types::OperatorRoleIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UserIdentifier: Option<crate::v5_0::common::UserIdentifierType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaVisualIdentificationChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlatformIdentity: Option<crate::v5_0::types::PlatformIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RadarCrossSection: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"Specifies how dynamic the returned Weapon Engagement Zone (WEZ) will be."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MaWezChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StaticWez: Option<crate::v5_0::types::MaWezStaticType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlannedStaticWez: Option<crate::v5_0::types::MaWezPlannedStaticType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PredictedWez: Option<crate::v5_0::types::MaWezPredictedType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlannedPredictedWez: Option<crate::v5_0::types::MaWezPlannedPredictedType>,
}

#[doc = r#"Indicates choices for sensor collection maneuver constraints."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ManeuverConstraintsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BasicManeuverConstraints: Option<crate::v5_0::types::BasicManeuverConstraintsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PredictedManeuverConstraints: Option<crate::v5_0::types::OrbitalManeuverDetailsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MeasurementKinematicsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LosAzEl: Option<crate::v5_0::types::LosMeasurementWithUncertaintyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LosEquatorial: Option<crate::v5_0::types::EquatorialKinematicsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Orbital: Option<crate::v5_0::choices::OrbitalKinematicsChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Wgs: Option<crate::v5_0::types::KinematicsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EcefKinematics: Option<crate::v5_0::types::EcefKinematicsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MissionEnvironmentConstraintTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedEntity: Option<crate::v5_0::types::ConstrainedEntityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedOpPoint: Option<crate::v5_0::types::ConstrainedOpPointType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedOpLine: Option<crate::v5_0::types::ConstrainedOpLineType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedOpZone: Option<crate::v5_0::types::ConstrainedOpZoneType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedOpVolume: Option<crate::v5_0::types::ConstrainedOpVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub System: Option<crate::v5_0::types::SystemStatusMdt>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RiskAdjustment: Option<crate::v5_0::types::RequirementRiskAdjustmentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Parameter: Option<crate::v5_0::types::ParameterAssertType>,
}

#[doc = r#"Defines the class of object for which Mission Environment Object parameters support individual settings on specific instances."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MissionEnvironmentObjectClassTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Entity: Option<crate::v5_0::types::MissionEnvironmentObjectValueEntityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub System: Option<crate::v5_0::types::MissionEnvironmentObjectValueSystemType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPoint: Option<crate::v5_0::types::MissionEnvironmentObjectValueOpPointType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpLine: Option<crate::v5_0::types::MissionEnvironmentObjectValueOpLineType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZone: Option<crate::v5_0::types::MissionEnvironmentObjectValueOpZoneType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolume: Option<crate::v5_0::types::MissionEnvironmentObjectValueOpVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Task: Option<crate::v5_0::types::MissionEnvironmentObjectValueTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Effect: Option<crate::v5_0::types::MissionEnvironmentObjectValueEffectType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Action: Option<crate::v5_0::types::MissionEnvironmentObjectValueActionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Response: Option<crate::v5_0::types::MissionEnvironmentObjectValueResponseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MissionPlanActivationDetailsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByMissionPlan: Option<crate::v5_0::types::MissionPlanActivationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BySubPlan: Option<crate::v5_0::types::MissionPlanSubplanActivationType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MissionPlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MissionPlanningAutonomyResponseChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutonomousPlanningAction: Option<Vec<crate::v5_0::types::PlanningAllowedEscalationType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AlertOnly: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MissionPlanningByResultAutonomousActionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlanningAllowed: Option<Vec<crate::v5_0::types::PlanningAllowedType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AlertOnly: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ModeSInterrogatorAddressTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InterrogatorIdentifier: Option<crate::v5_0::common::InterrogatorIdentifierType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SurveillanceIdentifier: Option<crate::v5_0::common::SurveillanceIdentifierType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MtiTargetClassTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MtiTargetClass: Option<crate::v5_0::enums::MtiTargetClassEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ForeignClass: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MustFlyLocationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPointId: Option<crate::v5_0::types::OpPointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpLineId: Option<crate::v5_0::types::OpLineIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZoneId: Option<crate::v5_0::types::OpZoneIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolumeId: Option<crate::v5_0::types::OpVolumeIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Point: Option<crate::v5_0::types::Point3DType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ZoneTarget: Option<crate::v5_0::types::ZoneExternalType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LineTarget: Option<crate::v5_0::types::LineTargetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VolumeTarget: Option<crate::v5_0::choices::OpVolumeType>,
}

#[doc = r#"The NameValuePairValue is used to report the value of a single status attribute.  This type is used to provide status for unique attributes that cannot be reported with other types or structures."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct NameValuePairValueTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BooleanValue: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByteValue: Option<i8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnsignedByteValue: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ShortValue: Option<i16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnsignedShortValue: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IntValue: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UnsignedIntValue: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LongValue: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FloatValue: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DoubleValue: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StringValue: Option<crate::v5_0::common::VisibleString256Type>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct NavigationSourceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanNavigation: Option<crate::v5_0::types::MissionPlanNavigationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FixedNavigation: Option<crate::v5_0::types::Point3DType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ManualNavigation: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoPilotNavigation: Option<crate::v5_0::enums::AutopilotModeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RelativeNavigation: Option<crate::v5_0::types::RelativeNavigationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SlavedNavigation: Option<crate::v5_0::types::SlavedNavigationType>,
}

#[doc = r#"Indicates the network endpoint (IP address) and its related network information."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct NetworkEndpointTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ipv4Endpoint: Option<crate::v5_0::types::Ipv4EndpointType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ipv6Endpoint: Option<crate::v5_0::types::Ipv6EndpointType>,
}

#[doc = r#"Encoding types for CVEnumISMNonIC Version 1 controlled vocabulary enumerations.  Derived from the CVEnumISMNonIC.xml CVE.(U) All currently valid Non-IC markings from the published register
						   PERMISSIBLE VALUES
	
						   The permissible values for this simple type are defined in the Controlled Value Enumeration:
	
						   CVEnumISMNonIC.xml"#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct NonIcMarkingsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StandardMarking: Option<crate::v5_0::enums::NonIcMarkingsEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AlternateMarking: Option<crate::v5_0::common::NonIcMarkingsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ObjectKinematicsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InertialState: Option<Vec<crate::v5_0::types::InertialStateType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitKinematics: Option<crate::v5_0::types::OrbitKinematicsType>,
}

#[doc = r#"Allows for a sibling operational constraint to be weighted by a discrete value or range threshold."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OpConstraintWeightingValueTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Discrete: Option<crate::v5_0::common::PercentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Range: Option<crate::v5_0::types::PercentRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Likelihood: Option<crate::v5_0::enums::LikelihoodEnum>,
}

#[doc = r#"A list of unique ID indicating the op type."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OpIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpLineId: Option<crate::v5_0::types::OpLineIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPointId: Option<crate::v5_0::types::OpPointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZoneId: Option<crate::v5_0::types::OpZoneIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolumeId: Option<crate::v5_0::types::OpVolumeIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpRoutingId: Option<crate::v5_0::types::OpRoutingIdType>,
}

#[doc = r#"Container object for the different types of OpPoint*Enums.  A separate enum applies to each of the choice types."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OpPointCategoriesTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub General: Option<crate::v5_0::enums::OpPointGeneralEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Hazard: Option<crate::v5_0::enums::OpPointHazardEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Reference: Option<crate::v5_0::enums::OpPointReferenceEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Station: Option<crate::v5_0::enums::OpPointStationEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Emergency: Option<crate::v5_0::enums::OpPointEmergencyEnum>,
}

#[doc = r#"Container object for the different types of OpPoint*Enums.  A separate enum applies to each of the choice types."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OpPointCategoriesUniqueDataTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Emergency: Option<crate::v5_0::types::EmergencyReferencePointType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Hazard: Option<crate::v5_0::types::Link16HazardType>,
}

#[doc = r#"Choice of either relative or geospatial position of the OpPoint."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OpPointChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Point: Option<crate::v5_0::types::OpPointPositionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RelativePoint: Option<crate::v5_0::types::Point2DRelativeType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OpPointReferenceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StoredObject: Option<crate::v5_0::choices::GeoLocatedStoredObjectType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StoredObjectRef: Option<crate::v5_0::choices::GeoLocatedObjectType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DataLinkObject: Option<crate::v5_0::types::DataLinkIdentifierPet>,
}

#[doc = r#"An operational volume comprises a three dimensional region of space."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OpVolumeTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeometricVolume: Option<crate::v5_0::types::GeometricVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeocentricVolume: Option<crate::v5_0::types::GeocentricVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitRegime: Option<crate::v5_0::types::OrbitRegimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitAltitude: Option<crate::v5_0::enums::OrbitAltitudeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Qualitative: Option<crate::v5_0::enums::OrbitQualitativeEnum>,
}

#[doc = r#"Container for parameters that are unique to a specific enumeration in OpZoneCategoryEnum.  For example, KeepIn enumeration can have amplifying information such as entry and exit restrictions of the zone."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OpZoneCategoryTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConstrainedEntryExit: Option<crate::v5_0::types::ConstrainedEntryExitType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FilterArea: Option<Vec<crate::v5_0::types::OpZoneFilterAreaPet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Jamming: Option<crate::v5_0::types::OpZoneJammingType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub KeepIn: Option<crate::v5_0::types::IngressEgressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissileLaunchPoint: Option<crate::v5_0::types::OpZoneMissileDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NoFire: Option<crate::v5_0::types::OpZoneNoFireType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NoFly: Option<crate::v5_0::types::OpZoneNoFlyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VehicleConfiguration: Option<crate::v5_0::types::VehicleCommandDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeaponRestriction: Option<crate::v5_0::types::OpZoneWeaponRestrictionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeatherConditions: Option<crate::v5_0::types::OpZoneWeatherType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OperatorNotificationActionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutonomousActions: Option<Vec<crate::v5_0::types::OperatorActionAutonomousType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ControlledActions: Option<crate::v5_0::types::OperatorActionControlledType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OperatorRoleTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OperatorRoleId: Option<crate::v5_0::types::OperatorRoleIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NonOperatorIdentifier: Option<crate::v5_0::types::SystemServiceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitActivityPlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitActivityPlanCommandId: Option<crate::v5_0::types::OrbitActivityPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitActivityPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"This type provides the details of an on-orbit event which results in physical damage, whether in whole or in part, of an object or multiple objects."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitBreakupEventDetailsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalConjunctionId: Option<crate::v5_0::types::OrbitalConjunctionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Collision: Option<crate::v5_0::types::OrbitCollisionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SingleObjectBreakup: Option<crate::v5_0::types::OrbitObjectBreakupType>,
}

#[doc = r#"Indicates a number of specific maneuvers to reach a new orbit."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitChangeChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificOrbit: Option<crate::v5_0::types::CoeOrbitType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificPosition: Option<crate::v5_0::types::OrbitalVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Rendezvous: Option<crate::v5_0::types::RsoApproachType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProximityOperations: Option<crate::v5_0::types::ProximityOperationsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Longitude: Option<crate::v5_0::common::AngleType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SemimajorAxis: Option<crate::v5_0::common::DistanceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Inclination: Option<crate::v5_0::common::AngleHalfPositiveType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Eccentricity: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RaceTrack: Option<crate::v5_0::types::RaceTrackOrbitType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitDurationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub Time: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NumberOfOrbits: Option<u32>,
}

#[doc = r#"Defines choice for replacement or modification of an orbit kinematics sequence."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitKinematicsSequenceReplaceOrModifyChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReplaceOrbitKinematicsSequence: Option<crate::v5_0::types::OrbitKinematicsSequenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ModifyOrbitKinematicsSequence: Option<crate::v5_0::types::OrbitKinematicsSequenceModificationDetailsType>,
}

#[doc = r#"Defines the source from which to get Orbit Kinematics."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitKinematicsSourceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByPlanId: Option<crate::v5_0::types::OrbitPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByElementSet: Option<crate::v5_0::choices::OrbitalElementSetSourceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByEphemeris: Option<crate::v5_0::choices::OrbitalEphemerisSourceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitPlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitPlanCommandId: Option<crate::v5_0::types::OrbitPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"Defines choice for location to insert orbit kinematics sequence."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitSequenceInsertionPositionChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InsertAtStart: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InsertAfterOrbitKinematicsSequenceId: Option<crate::v5_0::types::OrbitKinematicsSequenceIdType>,
}

#[doc = r#"Indicates the specific details of the object that is to be analyzed for close approaches."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalAnalysisObjectTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpecificObject: Option<crate::v5_0::types::OrbitalObjectKinematicsSourceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WorkingSobId: Option<crate::v5_0::types::WorkingSobIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DebrisCloudId: Option<crate::v5_0::types::OrbitalDebrisCloudIdType>,
}

#[doc = r#"Indicates the collection of Orbital Debris estimate information."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalDebrisEstimateTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FileMetadataId: Option<crate::v5_0::types::FileMetadataIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DebrisCloud: Option<Vec<crate::v5_0::types::OrbitalDebrisCloudType>>,
}

#[doc = r#"Indicates the source of the element set kinematics data."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalElementSetSourceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemElementSetId: Option<crate::v5_0::types::SystemOrbitalElementSetIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityElementSetId: Option<crate::v5_0::types::EntityOrbitalElementSetIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ExplicitElementSet: Option<crate::v5_0::types::TleBaseType>,
}

#[doc = r#"Indicates the ephemeris expressed in one of several orbital kinematics standards."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalEphemerisChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StandardFrame: Option<crate::v5_0::choices::OrbitalKinematicsStandardEphemerisType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitingObjectRelative: Option<crate::v5_0::types::OrbitalKinematicsRelativeEphemerisType>,
}

#[doc = r#"Indicates the source of the ephemeris kinematics data."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalEphemerisSourceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemEphemerisId: Option<crate::v5_0::types::SystemOrbitalEphemerisIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityEphemerisId: Option<crate::v5_0::types::EntityOrbitalEphemerisIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalEphemeris: Option<crate::v5_0::choices::OrbitalEphemerisChoiceType>,
}

#[doc = r#"Indicates the kinematics expressed in one of several orbital kinematics standards."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalKinematicsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StandardFrame: Option<crate::v5_0::choices::OrbitalKinematicsStandardFrameChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitingObjectRelative: Option<crate::v5_0::types::OrbitalKinematicsObjectRelativeType>,
}

#[doc = r#"Provides the choice of orbital kinematics reference frames."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalKinematicsFrameChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalElements: Option<crate::v5_0::types::TleWithParametersType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub J2k: Option<crate::v5_0::types::J2kKinematicsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Gcrs: Option<crate::v5_0::types::GcrsKinematicsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Bcrs: Option<crate::v5_0::types::BcrsKinematicsType>,
}

#[doc = r#"Indicates ephemeris expressed in terms of a standard reference frame centered on a celestial object."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalKinematicsStandardEphemerisTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub J2kStateVector: Option<Vec<crate::v5_0::types::J2kKinematicsType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GcrsStateVector: Option<Vec<crate::v5_0::types::GcrsKinematicsType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BcrsStateVector: Option<Vec<crate::v5_0::types::BcrsKinematicsType>>,
}

#[doc = r#"Provides the choice of kinematics in terms of a standard coordinate reference frame."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalKinematicsStandardFrameChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub J2k: Option<crate::v5_0::types::J2kKinematicsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Gcrs: Option<crate::v5_0::types::GcrsKinematicsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Bcrs: Option<crate::v5_0::types::BcrsKinematicsType>,
}

#[doc = r#"Indicates the choice between two-line element kinematic data or state vector (ECI J2K) kinematic data."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalKinematicsTleSvTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StateVector: Option<crate::v5_0::types::J2kKinematicsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Tle: Option<crate::v5_0::types::TleBaseType>,
}

#[doc = r#"Indicates orbital surveillance instructions and information to the sensor to enable appropriate sensor set-up and data collection to meet the orbital surveillance collection need."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalSurveillanceCollectionRequirementsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MetricCollection: Option<crate::v5_0::types::MetricCollectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Search: Option<crate::v5_0::types::SpeedRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitDetermination: Option<crate::v5_0::types::OrbitAccuracyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Characterization: Option<crate::v5_0::choices::SensorCharacterizationChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MultiObject: Option<crate::v5_0::types::MultiObjectType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ManeuverDetection: Option<crate::v5_0::types::ManeuverDetectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DeploymentDetection: Option<crate::v5_0::types::DeploymentDetectionType>,
}

#[doc = r#"Specifies span of time for individual collection based on duration or rotational periods of target."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalSurveillanceSensorMinimumCollectionRequirementsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TargetRotationalPeriods: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub Time: Option<chrono::TimeDelta>,
}

#[doc = r#"Indicates the expected size of the smallest target for the task (or threshold for search) in either physical area  or apparent size appropriate to the phenomenology (e.g., radar cross section)."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalSurveillanceSensorMinimumSizeTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RadarCrossSection: Option<crate::v5_0::types::PercentileRcstype>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VisualMagnitude: Option<crate::v5_0::common::VisualMagnitudeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Area: Option<crate::v5_0::common::AreaType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Intensity: Option<f64>,
}

#[doc = r#"Indicates the target of the Orbital Surveillance Sensor Task."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalSurveillanceSensorTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PointList: Option<crate::v5_0::choices::SensorPointListType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ElementSetCloud: Option<crate::v5_0::types::ElementSetCloudType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ObjectBased: Option<crate::v5_0::types::OrbitalSurveillanceObjectsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LocationBased: Option<crate::v5_0::types::OrbitalSurveillanceLocationTargetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SensorCentricVolume: Option<crate::v5_0::types::SourceCoverageType>,
}

#[doc = r#"Indicates a SubCapability of the Orbital Surveillance Capability, the second tier in the taxonomy of Orbital Surveillance.  For Orbital Surveillance, the second tier is the desired outcome of the collection.  See enumeration annotations for further details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalSurveillanceSubCapabilityDetailsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Search: Option<crate::v5_0::types::OrbitalSurveillanceSearchType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitAccuracy: Option<crate::v5_0::types::OrbitAccuracyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Characterization: Option<crate::v5_0::choices::CharacterizationObjectiveType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MultiObject: Option<crate::v5_0::types::MultiObjectType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ManeuverDetection: Option<crate::v5_0::types::ManeuverDetectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DeploymentDetection: Option<crate::v5_0::types::DeploymentDetectionType>,
}

#[doc = r#"Indicates the target of the Orbital Surveillance Task as either object based, location based, or zone based."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OrbitalSurveillanceTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ObjectBased: Option<crate::v5_0::types::OrbitalSurveillanceObjectsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LocationBased: Option<crate::v5_0::types::OrbitalSurveillanceLocationTargetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ZoneBased: Option<crate::v5_0::types::OrbitalSurveillanceZoneTargetType>,
}

#[doc = r#"Encoding types for CVEnumISMCATOwnerProducer Version 2 controlled vocabulary enumerations.  Derived from the CVEnumISMCATOwnerProducer.xml CVE.(U) 
				  FGI, followed by all currently valid GENC trigraphs in alphabetical order by trigraph, 
				  followed by all currently valid CAPCO Coalition tetragraphs in alphabetical order by tetragraph.

						   PERMISSIBLE VALUES

						   The permissible values for this simple type are defined in the Controlled Value Enumeration:

						   CVEnumISMCATOwnerProducer.xml"#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct OwnerProducerChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GovernmentIdentifier: Option<crate::v5_0::enums::OwnerProducerEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NatoSpecialWord: Option<crate::v5_0::common::NatoSpecialWordsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ParameterValueTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Value: Option<crate::v5_0::common::VisibleString256Type>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReturnToDefault: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"Specify a section of a path, by time or by segments."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PathConstraintsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SegmentWindow: Option<crate::v5_0::types::SegmentRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TimeWindow: Option<crate::v5_0::types::TimeWindowType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PathSegmentSpeedChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpeedValue: Option<crate::v5_0::types::PathSegmentSpeedValueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MachValue: Option<crate::v5_0::common::MachType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PlanActivationAutonomyTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByMissionPlan: Option<crate::v5_0::types::MissionPlanActivationSettingType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BySubPlan: Option<Vec<crate::v5_0::types::SubPlanActivationSettingType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PlanReferenceIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanId: Option<crate::v5_0::types::MissionPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TaskPlanId: Option<crate::v5_0::types::TaskPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitPlanId: Option<crate::v5_0::types::OrbitPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitActivityPlanId: Option<crate::v5_0::types::OrbitActivityPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RoutePlanId: Option<crate::v5_0::types::RoutePlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteActivityPlanId: Option<crate::v5_0::types::RouteActivityPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommScheduleAllocationId: Option<crate::v5_0::types::CommScheduleAllocationIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActivityPlanId: Option<crate::v5_0::types::ActivityPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EffectPlanId: Option<crate::v5_0::types::EffectPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActionPlanId: Option<crate::v5_0::types::ActionPlanIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ResponsePlanId: Option<crate::v5_0::types::ResponsePlanIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PlanWindowModificationTypeChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Window: Option<crate::v5_0::types::DateTimeRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub TimeOffset: Option<chrono::TimeDelta>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PlanningByCaseTriggerTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityAdded: Option<crate::v5_0::types::CapabilityTaxonomyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityFailure: Option<crate::v5_0::types::CapabilityTaxonomyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommsLost: Option<crate::v5_0::types::CommsLostTriggerDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DmpiOverDesignation: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DmpiUnderDesignation: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EnduranceLow: Option<crate::v5_0::types::EnduranceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OffRoute: Option<crate::v5_0::types::ThresholdOffRouteTriggerDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProximityConflict: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReleasePointOutsideLar: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteConflict: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteVulnerability: Option<crate::v5_0::types::PlanVulnerabilityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemStateChange: Option<crate::v5_0::types::SystemStateFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementAdded: Option<crate::v5_0::types::RequirementTriggerType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementDependencyFailed: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementDropped: Option<crate::v5_0::types::RequirementTriggerType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementFailed: Option<crate::v5_0::types::RequirementFailedTriggerType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementChange: Option<crate::v5_0::types::RequirementTriggerType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementTiming: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ZoneViolation: Option<crate::v5_0::types::ZoneViolationTriggerDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitConflict: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OffPlannedOrbit: Option<crate::v5_0::types::ThresholdOffOrbitTriggerDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpacecraftEnduranceLow: Option<crate::v5_0::types::SatelliteEnduranceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpacecraftProximityConflict: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ResponseId: Option<crate::v5_0::types::ResponseIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PlanningByResultTriggerTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReplanRequired: Option<Vec<crate::v5_0::enums::PlanTypeEnum>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VulnerabilityChanged: Option<crate::v5_0::types::PlanVulnerabilityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementUnallocated: Option<crate::v5_0::types::RequirementTriggerType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PlanningPointTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Specific: Option<crate::v5_0::types::PlanningLocationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPointId: Option<crate::v5_0::types::OpPointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Time: Option<chrono::DateTime<chrono::Utc>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PlatformFunctionStatusCategoryTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Air: Option<crate::v5_0::enums::PlatformFunctionAirEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SeaSurface: Option<crate::v5_0::enums::PlatformFunctionSeaSurfaceEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ground: Option<crate::v5_0::enums::PlatformFunctionGroundEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ew: Option<crate::v5_0::enums::PlatformFunctionEwEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PmopSequenceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PmopSequencePhase: Option<Vec<crate::v5_0::enums::PmopSequenceEnum>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PmopSequenceLength: Option<u32>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoAirTargetVolumeCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirVolume: Option<crate::v5_0::choices::PoAirTargetVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoAirTargetVolumeTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirVolumeSensorReferenced: Option<crate::v5_0::types::PoAirVolumeSensorReferencedType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirVolumeLocation: Option<crate::v5_0::types::ZoneType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoAngleConstraintControlsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Controls: Option<crate::v5_0::types::PoConstraintControlsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Setting: Option<crate::v5_0::types::AnglePairType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoCollectionPatternConstraintControlsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Controls: Option<crate::v5_0::types::PoConstraintControlsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Setting: Option<crate::v5_0::enums::CollectionPatternEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::PoCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::PoActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentFStopSettingsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FStopSetting: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoFStop: Option<bool>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentFStopTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FixedAperture: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VariableAperture: Option<crate::v5_0::types::PoComponentFStopVariableType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentLensAssemblyFieldOfViewTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FixedFov: Option<crate::v5_0::common::AngleQuarterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Zoom: Option<crate::v5_0::types::PoComponentLensAssemblyZoomType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsBandpassFrequencyTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FilterBandSetting: Option<crate::v5_0::types::FrequencyRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoFilterBand: Option<bool>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocalPlaneArrayCollectionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub CollectionTimeSetting: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CollectionTimeControls: Option<crate::v5_0::types::ComponentControlsBType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionDataTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NucTableNumber: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NucReferenceOffset: Option<crate::v5_0::types::FocalPlaneArrayNonUniformityCorrectionReferenceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NucSetting: Option<crate::v5_0::choices::PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NucControls: Option<crate::v5_0::types::ComponentControlsBType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocalPlaneArrayOpticalBandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BandSetting: Option<crate::v5_0::types::FrequencyRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BandControls: Option<crate::v5_0::types::ComponentControlsBType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocalPlaneArrayPixelAggregationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AggregationSetting: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AggregationControls: Option<crate::v5_0::types::ComponentControlsBType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocalPlaneArrayPixelPolarityTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PolaritySetting: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PolarityControls: Option<crate::v5_0::types::ComponentControlsBType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocalPlaneArrayPixelScalingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ScalingSettings: Option<crate::v5_0::types::PoComponentSettingsFocalPlaneArrayPixelScalingSettingsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ScalingControls: Option<crate::v5_0::types::ComponentControlsBType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocalPlaneArrayScanDirectionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DirectionSetting: Option<crate::v5_0::enums::RelativeDirectionEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DirectionControls: Option<crate::v5_0::types::ComponentControlsBType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocalPlaneArrayTimeDelayIntegrationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TdiSetting: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TdiControls: Option<crate::v5_0::types::ComponentControlsBType>,
}

#[doc = r#"The position to start the focus lens at when starting the focus sweep."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocusSweepSettingsStartingPositionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Percentage: Option<crate::v5_0::common::PercentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NumberOfSteps: Option<u32>,
}

#[doc = r#"The increment used to move the focus lens between steps."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocusSweepSettingsStepIncrementTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Percentage: Option<crate::v5_0::common::PercentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NumberOfSteps: Option<u32>,
}

#[doc = r#"The time required of each step in the focus sweep.  Generally only specified for line array sensors as this step time is fixed for grid array sensors."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsFocusSweepSettingsStepTimeTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NumberOfLines: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CollectionTime: Option<i64>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsLensAssemblyFieldOfViewTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FovSetting: Option<crate::v5_0::common::AngleQuarterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoZoom: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IncrementalChange: Option<crate::v5_0::types::IncrementalChangeType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsLensAssemblyFocusTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FocusSetting: Option<crate::v5_0::common::PercentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoFocus: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoFocusZoom: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IncrementalChange: Option<crate::v5_0::types::IncrementalChangeType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsOutputProductSettingsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductSettings: Option<crate::v5_0::types::PoComponentSettingsProductSettingsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoOutputProductSettings: Option<bool>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsProcessingStageSettingsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProcessingSettings: Option<crate::v5_0::types::PoComponentSettingsProcessingStageProcessingSettingsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoProcessingSettings: Option<bool>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentSettingsProductGeneratorSettingsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeneratorSettings: Option<crate::v5_0::types::PoComponentSettingsGeneratorSettingsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoGeneratorSettings: Option<bool>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentStatusFocalPlaneArrayNonUniformityCorrectionDataTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NucTableNumber: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NucReferenceOffset: Option<crate::v5_0::types::FocalPlaneArrayNonUniformityCorrectionReferenceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoComponentStatusLensAssemblyFocusTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FocusSetting: Option<crate::v5_0::common::PercentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoFocus: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AutoFocusZoom: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ChangeType: Option<crate::v5_0::enums::IncrementalChangeEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoSlantRangeConstraintControlsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Controls: Option<crate::v5_0::types::PoConstraintControlsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Setting: Option<crate::v5_0::types::DistanceConstraintsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoSweepSpeedConstraintControlsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Controls: Option<crate::v5_0::types::PoConstraintControlsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Setting: Option<crate::v5_0::common::SpeedType>,
}

#[doc = r#"Indicates the position or location to point the XX Subsystem to do a XX collection, track, or search as part of an XX Activity. If the XX Subsystem cannot point itself, then the TurretSlaved Type is used. If the system wants to allow the subsystem to control its own LOS then the ActivitySlavedID is used. FixedPointing is used to point to a predetermined location defined by its Enum values."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PoTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Geospatial: Option<Vec<crate::v5_0::choices::TargetType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Pointed: Option<crate::v5_0::types::TurretCommandPositionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LosOption: Option<crate::v5_0::choices::LosDType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Volume: Option<Vec<crate::v5_0::choices::PoAirTargetVolumeCommandType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TurretSlaved: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActivitySlavedId: Option<crate::v5_0::types::ActivityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FixedPointing: Option<crate::v5_0::enums::FixedPointingEnum>,
}

#[doc = r#"Specifies a location either as a geospatial location or a location relative to a separately defined reference frame."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PointChoice3DTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AbsolutePoint: Option<crate::v5_0::types::Point3DType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RelativePoint: Option<crate::v5_0::types::Point3DRelativeType>,
}

#[doc = r#"Specifies a location either as a geospatial location or a location relative to a separately defined reference frame."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PointChoice4DTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AbsolutePoint: Option<crate::v5_0::types::Point4DType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RelativePoint: Option<crate::v5_0::types::Point4DRelativeType>,
}

#[doc = r#"Specifies a location either as a geospatial location or a location relative to a separately defined reference frame."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PointChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AbsolutePoint: Option<crate::v5_0::types::Point2DType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RelativePoint: Option<crate::v5_0::types::Point2DRelativeType>,
}

#[doc = r#"Indicates the position or location to point the XX Subsystem to do a XX collection, track, or search as part of an XX Activity. If the XX Subsystem cannot point itself, then the TurretSlaved Type is used. If the system wants to allow the subsystem to control its own LOS then the ActivitySlavedID is used. FixedPointing is used to point to a predetermined location defined by its Enum values."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PointingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Geospatial: Option<Vec<crate::v5_0::choices::TargetType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LosOption: Option<crate::v5_0::choices::LosDType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Volume: Option<Vec<crate::v5_0::choices::PoAirTargetVolumeCommandType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TurretSlaved: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActivitySlavedId: Option<crate::v5_0::types::ActivityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FixedPointing: Option<crate::v5_0::enums::FixedPointingEnum>,
}

#[doc = r#"Specifies a polygon by geospatial locations or as locations relative to a separately defined reference frame."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PolygonPointChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Point2D: Option<Vec<crate::v5_0::types::Point2DType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RelativePolygon: Option<crate::v5_0::types::PolygonRelativeType>,
}

#[doc = r#"Indicates the source of position data."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PositionSourceIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SubsystemId: Option<crate::v5_0::types::SubsystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ServiceId: Option<crate::v5_0::types::ServiceIdType>,
}

#[doc = r#"This element defines a filter which can be applied to any product regardless of type."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ProductFilterTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Source: Option<crate::v5_0::types::SourceFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Geospatial: Option<crate::v5_0::choices::LocationFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SourceGeospatial: Option<crate::v5_0::types::SourceLocationFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FileFormat: Option<crate::v5_0::choices::FileFormatType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SecurityInformation: Option<crate::v5_0::types::SecurityInformationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DateTimeRange: Option<crate::v5_0::types::DateTimeRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub XPath: Option<crate::v5_0::common::VisibleString1024Type>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductType: Option<crate::v5_0::enums::ProductTypeEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ProductGeospatialLocationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Point2D: Option<crate::v5_0::types::Point2DType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Zone: Option<crate::v5_0::types::ZoneType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RelativePosition: Option<crate::v5_0::types::RelativePositionType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ProductLocationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Network: Option<crate::v5_0::types::EndpointReferenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub System: Option<crate::v5_0::types::ProductSystemLocationType>,
}

#[doc = r#"Indicates when the product is needed by."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ProductNeededByTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AsSoonAsPossible: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AbsoluteTime: Option<chrono::DateTime<chrono::Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub RelativeToEventTime: Option<chrono::TimeDelta>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ProductParentTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementId: Option<crate::v5_0::choices::RequirementInstanceIdChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductProcessingRequestId: Option<Vec<crate::v5_0::types::RequestIdType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ProductReferenceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FileName: Option<crate::v5_0::common::FileNameType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductMetadataId: Option<crate::v5_0::types::ProductMetadataIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProductReferenceByParent: Option<crate::v5_0::types::ProductReferenceByParentType>,
}

#[doc = r#"This element defines a filter which can be applied to a specific product type."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ProductTypeFilterTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Source: Option<crate::v5_0::types::SourceFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Geospatial: Option<crate::v5_0::choices::LocationFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SourceGeospatial: Option<crate::v5_0::types::SourceLocationFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FileFormat: Option<crate::v5_0::choices::FileFormatType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SecurityInformation: Option<crate::v5_0::types::SecurityInformationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DateTimeRange: Option<crate::v5_0::types::DateTimeRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub XPath: Option<crate::v5_0::common::VisibleString1024Type>,
}

#[doc = r#"Indicates the choice of propagator types: A general Propagator or a VCM Propagator."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PropagatorChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeneralPropagator: Option<crate::v5_0::types::PropagatorType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VcmPropagator: Option<crate::v5_0::types::VcmPropagatorType>,
}

#[doc = r#"Indicates the choice of propagator settings. Specify an ID for an existing set of settings or manually specify a set."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct PropagatorSettingsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PropagatorSettingsId: Option<crate::v5_0::types::PropagatorSettingsIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PropagatorChoice: Option<crate::v5_0::choices::PropagatorChoiceType>,
}

#[doc = r#"This complex type provides the different types of proximity operations."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ProximityOrbitChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NaturalMotion: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ForcedMotion: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RBarPerch: Option<crate::v5_0::enums::RBarApproachEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VBarPerch: Option<crate::v5_0::enums::VBarApproachEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DeltaOrbitalPlaneTolerance: Option<crate::v5_0::types::AngleHalfPairType>,
}

#[doc = r#"Compares the length of the sequence formed by the specified Step to the value indicated by this choice."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct QueryCountValueTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Equals: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LessThan: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LessThanOrEqualTo: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GreaterThan: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GreaterThanOrEqualTo: Option<u32>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct QueryResultTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Message: Option<Vec<crate::v5_0::types::MessageType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Id: Option<Vec<crate::v5_0::types::IdType>>,
}

#[doc = r#"The mechanism by which queries navigate the data model of a UCI Message, which should be considered as a tree structure containing branch and leaf nodes.  Steps may either traverse up the tree (Ancestor and Parent), down the tree (Element, Descendant, and Child), or remain at the current context.  Steps are evaluated sequentially from the current context, with the default context of a query as the root of the tree, i.e. the top-level global element declaration.  The result of each Step forms a sequence of zero or more nodes that is then used as the input to the next Step, where each node of the input sequence is used as the current context with all sequences concatenated together, repeated until all Steps are evaluated.  Each node in this sequence is either a present optional field, a required field, or an item in a list.  For example, a Step that matches a list field with a length of two will result in a sequence of two nodes.  The resulting sequence is then evaluated by the query with a given operation.  Some operations, such as Equals, that operate on a single value are instead performed on each individual node in the sequence and the result is computed by the logical OR of all the results."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct QueryStepTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Element: Option<crate::v5_0::types::NamedElementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Root: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ancestor: Option<crate::v5_0::types::WildcardElementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Descendant: Option<crate::v5_0::types::WildcardElementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Parent: Option<crate::v5_0::types::WildcardElementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Child: Option<crate::v5_0::types::WildcardElementType>,
}

#[doc = r#"Defines the generic UCI Query Language (UQL) operations.  Each operation has a single input, a node that is the current context of the query, and will output either true or false.  When determining whether a Message should be sent in a corresponding status, it should be sent if the output of the query is true.  The data model of a UCI Message should be considered as a tree structure containing branch and leaf nodes.  The default context of a query is the Message, i.e. the top-level global element declaration.  Each UQL query is evaluated separately for each Message.  For more information on how the tree is evaluated, see the annotations in QueryStepType."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct QueryTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Equals: Option<crate::v5_0::types::QueryEqualsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LessThan: Option<crate::v5_0::types::QueryComparisonType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LessThanOrEqualTo: Option<crate::v5_0::types::QueryComparisonType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GreaterThan: Option<crate::v5_0::types::QueryComparisonType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GreaterThanOrEqualTo: Option<crate::v5_0::types::QueryComparisonType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ContainsCaseSensitive: Option<crate::v5_0::types::QueryContainsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ContainsCaseInsensitive: Option<crate::v5_0::types::QueryContainsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InstanceOf: Option<crate::v5_0::types::QueryInstanceOfType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Count: Option<crate::v5_0::types::QueryCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Exists: Option<crate::v5_0::types::QueryExistsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AnyMatch: Option<crate::v5_0::types::QueryMatchType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AllMatch: Option<crate::v5_0::types::QueryMatchType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub And: Option<Vec<crate::v5_0::types::QueryPet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Or: Option<Vec<crate::v5_0::types::QueryPet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Not: Option<crate::v5_0::types::QueryPet>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RadarAltimeterCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::RadarAltimeterCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::RadarAltimeterActivityCommandType>,
}

#[doc = r#"This is a switch that allows TargetType to be a sibling of SensorReferencedCoverageArea."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RadarPointingTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeospatialTarget: Option<crate::v5_0::choices::TargetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SensorReferencedCoverageArea: Option<crate::v5_0::types::SensorReferencedCoverageAreaType>,
}

#[doc = r#"Beam spoiling or taper to be applied to transmit or receive beam."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RadarSpoilTaperTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Taper: Option<crate::v5_0::types::RadarTaperType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Spoil: Option<crate::v5_0::types::RadarSpoilType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RadarTaperWeightingFunctionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StandardWeightingFunction: Option<crate::v5_0::enums::RadarWeightingFunctionsEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OtherTaper: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RangeElevationExtentChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Elevation: Option<crate::v5_0::types::AnglePairType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Range: Option<crate::v5_0::types::RangeExtentType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ReadinessTimeSpanTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByMission: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SubMission: Option<crate::v5_0::choices::ScheduleType>,
}

#[doc = r#"Container to provide the appropriate object that is the origin of a reference frame created with message ReferenceFrame."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ReferenceFrameObjectToFollowTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalReportId: Option<crate::v5_0::types::SignalReportIdType>,
}

#[doc = r#"Provides the object that is the origin of a reference frame. This allows defining an area around a point (object) that is not stationary, it moves along with the object so its definition is relative to that object."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ReferenceFrameOriginChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AlternateSource: Option<crate::v5_0::types::ForeignKeyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ObjectToFollowIdentifier: Option<crate::v5_0::choices::ReferenceFrameObjectToFollowType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeospatialPosition: Option<crate::v5_0::types::ReferenceFrameOriginType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub KinematicsReferenceFrameOrigin: Option<crate::v5_0::types::ReferenceFrameOriginKinematicsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ReferenceObjectTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPointId: Option<crate::v5_0::types::OpPointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RefuelCapabilityCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Boom: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Drogue: Option<crate::v5_0::types::RefuelCapabilityDrogueCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RefuelCapabilityStatusTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Boom: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Drogue: Option<crate::v5_0::types::RefuelCapabilityDrogueStatusType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RefuelCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::RefuelCapabilityExtendCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::RefuelActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RefuelConnectionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Boom: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Drogue: Option<crate::v5_0::types::RefuelActivityDrogueCommandType>,
}

#[doc = r#"Describes the relationship between two identified objects."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RelationshipTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EngagementStatus: Option<crate::v5_0::enums::ExternalCommandExecutionStateEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Pairing: Option<crate::v5_0::enums::PairingRelationshipEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Threat: Option<crate::v5_0::types::RelationshipThreatType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ElectronicWarfare: Option<crate::v5_0::types::RelationshipElectronicWarfareType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ControllingUnit: Option<crate::v5_0::types::RelationshipControllingUnitType>,
}

#[doc = r#"Encoding types for CVEnumISMCATRelTo Version 2 controlled vocabulary enumerations.  Derived from the CVEnumISMCATRelTo.xml CVE.(U) 
				  USA, followed by all currently valid GENC trigraphs except USA in alphabetical order by trigraph, 
				  followed by all currently valid CAPCO Coalition tetragraphs in alphabetical order by tetragraph.

						   PERMISSIBLE VALUES

						   The permissible values for this simple type are defined in the Controlled Value Enumeration:

						   CVEnumISMCATRelTo.xml"#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ReleasableToChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GovernmentIdentifier: Option<crate::v5_0::enums::ReleasableToEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NatoSpecialWord: Option<crate::v5_0::common::NatoSpecialWordsType>,
}

#[doc = r#"Provides a choice between event types to act as a trigger for an event-based repetition."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RepetitionEventTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PositionChange: Option<crate::v5_0::choices::RepetitionPositionChangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteEvent: Option<crate::v5_0::enums::RouteEventEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalEvent: Option<crate::v5_0::enums::OrbitalEventEnum>,
}

#[doc = r#"Provides a choice of position delta types."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RepetitionPositionChangeTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LosBearingElevation: Option<crate::v5_0::types::LosType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LosAzEl: Option<crate::v5_0::types::LosInertialAType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalRtn: Option<crate::v5_0::types::ThresholdOffOrbitTriggerDataType>,
}

#[doc = r#"Represents a Time-Based repetition."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RepetitionTimeBasedTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Continuous: Option<crate::v5_0::types::RepetitionContinuousType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Finite: Option<crate::v5_0::types::RepetitionFiniteType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Periodic: Option<crate::v5_0::types::RepetitionPeriodicType>,
}

#[doc = r#"Provides a choice between Time-Based and Event-Based Repetition."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RepetitionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TimeBased: Option<crate::v5_0::choices::RepetitionTimeBasedType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EventBased: Option<crate::v5_0::types::RepetitionEventBasedType>,
}

#[doc = r#"This type provides the correlation between an activity, a BIT, or a Calibration and a subsystem's RF_ResourceAllocationRequest."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RequestingFunctionIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActivityId: Option<crate::v5_0::types::ActivityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BitId: Option<crate::v5_0::types::BitIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CalibrationId: Option<crate::v5_0::types::CalibrationIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RequirementAssociationConstraintTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AllOrNothing: Option<crate::v5_0::types::AssociatedRequirementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EitherOr: Option<crate::v5_0::types::AssociatedRequirementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SameSystem: Option<crate::v5_0::types::AssociatedRequirementsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RequirementChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByType: Option<crate::v5_0::choices::RequirementTaxonomyChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByInstance: Option<crate::v5_0::choices::RequirementInstanceIdChoiceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RequirementInstanceIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EffectId: Option<crate::v5_0::types::EffectIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActionId: Option<crate::v5_0::types::ActionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TaskId: Option<crate::v5_0::types::TaskIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityCommandId: Option<crate::v5_0::types::CommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ResponseId: Option<crate::v5_0::types::ResponseIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RequirementMetricsCategoryTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CollectionMetrics: Option<crate::v5_0::types::CollectionTaskMetricsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StrikeMetrics: Option<crate::v5_0::types::StrikeTaskMetricsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommRelayMetrics: Option<crate::v5_0::types::CommRelayTaskMetricsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RequirementTaxonomyChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Effect: Option<crate::v5_0::enums::EffectTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Action: Option<crate::v5_0::enums::ActionTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Task: Option<crate::v5_0::enums::TaskTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityCommand: Option<crate::v5_0::enums::CapabilityTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Response: Option<crate::v5_0::enums::ResponseTypeEnum>,
}

#[doc = r#"Specifies the desired aspects of the spacecraft to be characterized."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ResolvedCharacterizationAspectCoverageTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BodyReference: Option<Vec<crate::v5_0::enums::BodyReferenceEnum>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Attitude: Option<crate::v5_0::types::QuaternionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AspectSpan: Option<crate::v5_0::common::AngleType>,
}

#[doc = r#"Allows a request or allocation to be directed to either RF or digital resources."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ResourceDefinitionChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AllocateRfResources: Option<crate::v5_0::types::ResourceDefinitionRfType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AllocateDigitalResources: Option<crate::v5_0::types::ResourceDefinitionDigitalType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ResponseCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::ResponseCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::ActivityCommandBaseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ResponseOptionTriggerTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Entity: Option<crate::v5_0::types::EntityFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub System: Option<crate::v5_0::types::SystemFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Requirement: Option<crate::v5_0::types::RequirementFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AccessAssessment: Option<crate::v5_0::types::AccessAssessmentFilterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Oloi: Option<crate::v5_0::types::OperatorLocationOfInterestClauseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ResponseCommand: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AnyMessage: Option<crate::v5_0::types::QueryMessageType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ResponseOptionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Effect: Option<crate::v5_0::enums::EffectTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EffectId: Option<crate::v5_0::types::EffectIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Action: Option<crate::v5_0::enums::ActionTypeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActionId: Option<crate::v5_0::types::ActionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Task: Option<crate::v5_0::choices::TaskResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TaskId: Option<crate::v5_0::types::TaskIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityCommand: Option<crate::v5_0::choices::CommandResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CapabilityCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ResponsePlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ResponsePlanCommandId: Option<crate::v5_0::types::ResponsePlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ResponsePlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RfThreadInstanceCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetupRfThreadInstance: Option<Vec<crate::v5_0::types::SetupRfThreadInstanceType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ModifyRfThreadInstance: Option<Vec<crate::v5_0::types::ModifyRfThreadInstanceType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RemoveRfThreadInstance: Option<Vec<crate::v5_0::types::RemoveRfThreadInstanceType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RfdGainSettingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GainRange: Option<crate::v5_0::types::GainRangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ValidGainSettings: Option<Vec<crate::v5_0::common::VisibleString32Type>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RouteActivityPlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteActivityPlanCommandId: Option<crate::v5_0::types::RouteActivityPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteActivityPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RoutePlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RoutePlanCommandId: Option<crate::v5_0::types::RoutePlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RoutePlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RuleResponseTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RequirementsTemplate: Option<crate::v5_0::types::ResponseTemplateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ActivatePlan: Option<crate::v5_0::types::MissionPlanActivationCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GenerateAlert: Option<crate::v5_0::types::ResponseAlertType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DoNothing: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"Indicates the subcapability, SAR or ISAR, for this command."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SarCapabilityCommandSubCapabilityTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Sar: Option<crate::v5_0::types::SarSubCapabilityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Isar: Option<crate::v5_0::types::IsarSubCapabilityType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SarCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::SarCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::SarActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SarDesiredWaveformTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WaveformType: Option<crate::v5_0::enums::SarWaveformSelectionEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ForeignWaveform: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"Indicates whether this is a SAR task or an ISAR task."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SarTaskTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Sar: Option<crate::v5_0::types::SarTargetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Isar: Option<crate::v5_0::choices::IsarTargetType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SarWaveformTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WaveformType: Option<crate::v5_0::enums::SarWaveformSelectionEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ForeignWaveform: Option<crate::v5_0::types::ForeignKeyType>,
}

#[doc = r#"Indicates the identity of an asset either by type or instance."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SatelliteIdentityChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByInstance: Option<crate::v5_0::choices::AssetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByType: Option<crate::v5_0::types::SatelliteIdentityType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ScheduleTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TimeSpan: Option<Vec<crate::v5_0::types::DateTimeRangeType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeekdayInterval: Option<Vec<crate::v5_0::types::WeekdayIntervalType>>,
}

#[doc = r#"Encoding types for CVEnumISMSCIControls Version 2 controlled vocabulary enumerations.  Derived from the CVEnumISMSCIControls.xml CVE.(U) All currently valid SCI controls from the published register
						   PERMISSIBLE VALUES

						   The permissible values for this simple type are defined in the Controlled Value Enumeration:

						   CVEnumISMSCIControls.xml"#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SciControlsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StandardCompartment: Option<crate::v5_0::enums::SciControlsEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SubCompartment: Option<crate::v5_0::common::SciControlsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SdaSpecialInstructionsConstraintTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub All: Option<crate::v5_0::types::SdaSpecialInstructionsSetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Any: Option<crate::v5_0::types::SdaSpecialInstructionsSetType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SecurityEventTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AccountAccess: Option<crate::v5_0::types::SecurityAccessType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Authentication: Option<crate::v5_0::types::SecurityAuthenticationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Integrity: Option<crate::v5_0::types::SecurityIntegrityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AuditManagement: Option<crate::v5_0::types::SecurityAuditManagementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FileManagement: Option<crate::v5_0::types::SecurityFileManagementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub KeyManagement: Option<crate::v5_0::types::SecurityKeyManagementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PolicyManagement: Option<crate::v5_0::types::SecurityPolicyManagementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub System: Option<crate::v5_0::types::SecuritySystemType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IntrusionDetection: Option<crate::v5_0::types::SecurityIntrusionDetectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Sanitization: Option<crate::v5_0::types::SecuritySanitizationType>,
}

#[doc = r#"Used to identify the RF payload resource which is the subject of an RF_ResourceAllocationRequest."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SelectPayloadResourceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PayloadResourceTypeIndex: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PayloadResourceInstanceIndex: Option<u32>,
}

#[doc = r#"Indicates collection requirements for each sensor characterization choice."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SensorCharacterizationChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PhemonemologySpecific: Option<crate::v5_0::choices::CharacterizationChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StabilityAndOrientationAssessment: Option<crate::v5_0::types::StabilityCharacterizationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StructureAssessment: Option<crate::v5_0::types::StructureAssessmentCharacterizationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IdentificationVerification: Option<crate::v5_0::types::IdentificationVerificationCharacterizationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OperationsChanges: Option<crate::v5_0::types::SatelliteOperationsChangesCharacterizationType>,
}

#[doc = r#"See the annotation in the associated message for an overall description of the message and this type."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SensorFieldOfRegardTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Extents: Option<crate::v5_0::types::FieldOfRegardExtentsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Volume: Option<crate::v5_0::types::GeometricVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GeoLongitude: Option<crate::v5_0::types::AnglePairType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SensorKinematicsChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Kinematics: Option<crate::v5_0::types::SignalNavDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalKinematics: Option<crate::v5_0::types::SensorKinematicsOrbitalType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EcefKinematics: Option<crate::v5_0::types::EcefSensorKinematicsType>,
}

#[doc = r#"This is a set of points or directions desired for a sensor collection. The sensor coordinates are the origin. This can be used to specify a sensor search pattern."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SensorPointListTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AzimuthElevationRangePointList: Option<Vec<crate::v5_0::types::AzimuthElevationRangePointType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RightAscensionDeclinationPointList: Option<Vec<crate::v5_0::types::RightAscensionDeclinationPointType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Point3Dlist: Option<Vec<crate::v5_0::types::Point3DType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ServiceConfigurationChangeTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ParameterChange: Option<Vec<crate::v5_0::types::ParameterAssertType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConfigurationFileReload: Option<Vec<crate::v5_0::common::AttributedUriType>>,
}

#[doc = r#"This generic type provides a choice for 3-dimensional shapes (e.g. spheres, cones, etc.)."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct Shape3DChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Sphere: Option<crate::v5_0::types::SphereType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Dome: Option<crate::v5_0::types::DomeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ellipsoid: Option<crate::v5_0::types::EllipsoidType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Cylinder: Option<crate::v5_0::types::CylinderType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Cone: Option<crate::v5_0::types::ConeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RectangularCone: Option<crate::v5_0::types::RectangularConeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ArcVolume: Option<crate::v5_0::types::ArcVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IncRaPeriodVolume: Option<crate::v5_0::types::IncRaPeriodVolumeType>,
}

#[doc = r#"Provides different status fields depending on the particular SupportCapability type providing the status."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SharedApertureSupportCapabilityStatusItemTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AntennaStatus: Option<crate::v5_0::types::AntennaStatusType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ResourceAllocatorStatus: Option<crate::v5_0::types::ResourceAllocatorStatusType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RfdcStatus: Option<crate::v5_0::types::RfdcStatusType>,
}

#[doc = r#"Indicates collection requirements to perform size estimation characterization in support a structure change detection sensor task."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SizeEstimationCharacterizationTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MinCollection: Option<crate::v5_0::choices::OrbitalSurveillanceSensorMinimumCollectionRequirementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SizeData: Option<crate::v5_0::enums::OrbitalSurveillanceSizeDataEnum>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SmtiCollectionConstraintsQualityTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Mtiirs: Option<crate::v5_0::common::NiirsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CircularErrorProbable90: Option<crate::v5_0::common::DistanceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SmtiCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::SmtiCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::SmtiActivityCommandType>,
}

#[doc = r#"A choice of solar radiation pressure (SRP) coefficient to use."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SolarRadiationPressureCoefficientChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VcmSolarRadiationPressureCoefficient: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReflectivityCoefficient: Option<f64>,
}

#[doc = r#"Indicates the System, Subsystem, or Service for which this applies."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SourceIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SubsystemId: Option<crate::v5_0::types::SubsystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ServiceId: Option<crate::v5_0::types::ServiceIdType>,
}

#[doc = r#"Defines the type that allows a choice of space weather data type: SpaceWeather message or static values."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SpaceWeatherDataChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SpaceWeatherValuesId: Option<crate::v5_0::types::SpaceWeatherIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StaticValues: Option<crate::v5_0::types::SpaceWeatherParameterType>,
}

#[doc = r#"Defines the type of geomagnetic index to use: Kp or Ap."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SpaceWeatherKpApChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Kp: Option<crate::v5_0::common::GeomagneticKpIndexType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ap: Option<crate::v5_0::common::GeomagneticApIndexType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct StoreCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NextStoreStation: Option<crate::v5_0::types::ForeignKeyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NextStoreType: Option<crate::v5_0::types::StoreType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OverrideLar: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OverrideAttitudeConstraints: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MasterArm: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReleaseConsent: Option<crate::v5_0::types::ReleaseConsentType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LarCalculationWindHold: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LarCalculationWindOverride: Option<crate::v5_0::types::Velocity2DType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VerifyInventory: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"What store type Mission or Carriage."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct StoreItemTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Mission: Option<crate::v5_0::types::StoreLoadoutMissionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Carriage: Option<crate::v5_0::types::StoreLoadoutCarriageType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct StoreLoadoutChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StoreList: Option<Vec<crate::v5_0::types::StoreLoadoutItemPet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Terminator: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct StrikeCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::StrikeCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::StrikeActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct StrikeTaskMetricsTargetingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Quality: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UncertaintyEllipse: Option<crate::v5_0::types::EllipseType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct StrikeTaskReleaseConstraintsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReleasePoint: Option<crate::v5_0::types::Point3DType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ReleaseArea: Option<crate::v5_0::types::AreaConstraintsType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct StrikeWeaponCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SelectForKeyLoad: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AssignTarget: Option<crate::v5_0::choices::GeoLocatedObjectType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeaponArm: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SelectForJettison: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GenerateDynamicLar: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SelectForRelease: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SelectAoCode: Option<crate::v5_0::types::AoCodeType>,
}

#[doc = r#"Indicates a task and requirements to enable characterization of or assessment of changes to the structure of a spacecraft."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct StructureAssessmentTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SizeEstimation: Option<crate::v5_0::types::SizeEstimationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Resolved: Option<crate::v5_0::types::ResolvedCharacterizationType>,
}

#[doc = r#"Indicates whether an UCI Entity subject or an UCI System subject is to be selected."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SubjectTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
}

#[doc = r#"Identifies specific BIT IDs or Fault codes relevant to this command."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SubsystemBitCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InitiateBitId: Option<Vec<crate::v5_0::types::BitIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CancelBitId: Option<Vec<crate::v5_0::types::BitIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ClearFaultCode: Option<Vec<crate::v5_0::common::VisibleString256Type>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SubsystemCalibrationCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InitiateCalibrationId: Option<crate::v5_0::types::CalibrationIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CancelCalibrationId: Option<crate::v5_0::types::CalibrationIdType>,
}

#[doc = r#"A choice type to allow further recursion or a terminator to signal the end of recursion."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SubsystemMaintenanceSubtestChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Subtest: Option<Vec<crate::v5_0::types::SubsystemMaintenanceTestPet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Terminator: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"A choice type to allow further recursion or a terminator to signal the end of recursion."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SubsystemMaintenanceSubtestCommandChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Subtest: Option<Vec<crate::v5_0::types::SubsystemMaintenanceTestCommandPet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Terminator: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"A choice type to allow further recursion or a terminator to signal the end of recursion."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SubsystemMaintenanceSubtestResultChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SubtestResultData: Option<Vec<crate::v5_0::types::SubsystemMaintenanceTestResultPet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Terminator: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SystemCharacteristicTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Identity: Option<crate::v5_0::choices::EntityIdentityChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PositionUncertainty: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "crate::v5_0::serde_utils::time_delta_opt")]
	pub PositionStaleness: Option<chrono::TimeDelta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PrioritizationList: Option<crate::v5_0::types::PrioritizationListValueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Behavior: Option<crate::v5_0::types::BehaviorType>,
}

#[doc = r#"Indicates the choice of how the ephemeris of the orbiting System will be obtained. These include from the element set results, the element set's ID, or the  kinematics vectors from a standard reference frame."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SystemEphemerisBasisChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ElementSet: Option<crate::v5_0::types::TleBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemElementSetId: Option<crate::v5_0::types::SystemOrbitalElementSetIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub KinematicVector: Option<crate::v5_0::choices::OrbitalKinematicsStandardFrameChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemVcmId: Option<crate::v5_0::types::SystemOrbitalVcmIdType>,
}

#[doc = r#"Indicates the initial conditions for a system estimation."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SystemEstimationStartTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub StartTime: Option<chrono::DateTime<chrono::Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteEstimationStart: Option<crate::v5_0::types::EstimationStartType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitEstimationStart: Option<crate::v5_0::types::OrbitEstimationStartType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SystemEstimationStopTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Time: Option<chrono::DateTime<chrono::Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RouteSegmentId: Option<crate::v5_0::types::SegmentIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitManeuverSegmentId: Option<crate::v5_0::types::OrbitManeuverSegmentIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct SystemManagementRequestTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetMode: Option<crate::v5_0::enums::MessageModeEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetIdentity: Option<crate::v5_0::types::SystemIdentityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetLink16Metadata: Option<crate::v5_0::types::Link16MetadataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetVoiceControl: Option<crate::v5_0::types::VoiceControlType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SetSensorEntityReporting: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VehicleSettings: Option<crate::v5_0::types::VehicleCommandDataType>,
}

#[doc = r#"Indicates the target of a TagAssociation, which could be a message or a string value."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct TagAssociationTargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByMessage: Option<crate::v5_0::types::AssociatedMessageType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ByValue: Option<crate::v5_0::types::SecureStringType>,
}

#[doc = r#"Indicates or references geospatial characteristics of a target."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct TargetTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OperatorLocationOfInterestId: Option<crate::v5_0::types::OperatorLocationOfInterestIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SignalId: Option<crate::v5_0::types::SignalIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpPointId: Option<crate::v5_0::types::OpPointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZoneId: Option<crate::v5_0::types::OpZoneIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolumeId: Option<crate::v5_0::types::OpVolumeIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpLineId: Option<crate::v5_0::types::OpLineIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PointTarget: Option<crate::v5_0::types::PointTargetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ZoneTarget: Option<crate::v5_0::types::ZoneExternalType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VolumeTarget: Option<crate::v5_0::choices::OpVolumeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LineTarget: Option<crate::v5_0::types::LineTargetType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct TaskPlanCommandIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TaskPlanCommandId: Option<crate::v5_0::types::TaskPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TaskPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanCommandId: Option<crate::v5_0::types::MissionPlanCommandIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub MissionPlanValidationCommandId: Option<crate::v5_0::types::CommandIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct TaskResponseTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirSample: Option<crate::v5_0::types::AirSampleTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Amti: Option<crate::v5_0::types::AmtiTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ao: Option<crate::v5_0::types::AoTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CargoDelivery: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Comint: Option<crate::v5_0::types::ComintTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommRelay: Option<crate::v5_0::types::CommRelayTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ea: Option<crate::v5_0::types::EaResponseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Esm: Option<crate::v5_0::types::EsmTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Flight: Option<crate::v5_0::types::FlightTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitChange: Option<crate::v5_0::types::OrbitChangeTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalSurveillance: Option<crate::v5_0::types::OrbitalSurveillanceTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Po: Option<crate::v5_0::types::PoTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Refuel: Option<crate::v5_0::types::RefuelTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Sar: Option<crate::v5_0::types::SarTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Smti: Option<crate::v5_0::types::SmtiTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Strike: Option<crate::v5_0::types::StrikeTaskWeaponListType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemDeployment: Option<crate::v5_0::types::SystemDeploymentTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TacticalOrder: Option<crate::v5_0::types::TacticalOrderTaskBaseType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeatherRadar: Option<crate::v5_0::common::EmptyType>,
}

#[doc = r#"Identifies the type of this Task instance. Note: When modifying this complexType (whether adding or removing choices), there are equivalent complexTypes that require the same modifications. Changes to this type may necessitate a modification to CapabilityTaxonomyType."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct TaskTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AirSample: Option<crate::v5_0::types::AirSampleTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Amti: Option<crate::v5_0::types::AmtiTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ao: Option<crate::v5_0::types::AoTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CargoDelivery: Option<crate::v5_0::choices::CargoDeliveryTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Comint: Option<crate::v5_0::types::ComintTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommRelay: Option<crate::v5_0::types::CommRelayTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CounterSpace: Option<crate::v5_0::types::CounterSpaceTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ea: Option<crate::v5_0::types::EaTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Esm: Option<crate::v5_0::types::EsmTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Flight: Option<crate::v5_0::types::FlightTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitChange: Option<crate::v5_0::types::OrbitChangeTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalSurveillance: Option<crate::v5_0::types::OrbitalSurveillanceTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OrbitalSurveillanceSensor: Option<crate::v5_0::types::OrbitalSurveillanceSensorTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Po: Option<crate::v5_0::types::PoTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Refuel: Option<crate::v5_0::types::RefuelTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Sar: Option<crate::v5_0::types::SarTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Smti: Option<crate::v5_0::types::SmtiTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Strike: Option<crate::v5_0::types::StrikeTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemDeployment: Option<crate::v5_0::types::SystemDeploymentTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TacticalOrder: Option<crate::v5_0::types::TacticalOrderTaskType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeatherRadar: Option<crate::v5_0::types::WeatherRadarTaskType>,
}

#[doc = r#"Provides a choice of timing constraints including repetitive timing and event based repetition."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct TimingConstraintsTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AsSoonAsPossible: Option<crate::v5_0::common::EmptyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TimeWindow: Option<crate::v5_0::types::TimeWindowType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeekdayInterval: Option<crate::v5_0::types::WeekdayIntervalType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Repetitive: Option<crate::v5_0::types::RepetitionConstraintsType>,
}

#[doc = r#"Choice between a Link 16 TN or UCI EntityID_Type value."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct TrackNumberOrEntityTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub EntityId: Option<crate::v5_0::types::EntityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TrackNumber: Option<crate::v5_0::types::Link16TrackIdentifierType>,
}

#[doc = r#"This element is used to specify whether a turn is a bank angle or turn radius."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct TurnGeometryChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TurnRadius: Option<crate::v5_0::common::DistanceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BankAngle: Option<crate::v5_0::common::AngleHalfType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct TurretCommandChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FixedMode: Option<crate::v5_0::enums::FixedPointingEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Position: Option<crate::v5_0::types::TurretCommandPositionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LosPosition: Option<crate::v5_0::choices::LosDType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Volume: Option<crate::v5_0::choices::PoAirTargetVolumeCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Geospatial: Option<crate::v5_0::choices::TargetType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct TurretReportPointingTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FixedPointing: Option<crate::v5_0::enums::FixedPointingEnum>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Dynamic: Option<crate::v5_0::types::TurretReportDynamicPointingType>,
}

#[doc = r#"The ID type for UCI IDs that correspond to a Validator."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ValidatorTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OperatorId: Option<crate::v5_0::types::OperatorIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub NonOperatorIdentifier: Option<crate::v5_0::types::SystemServiceType>,
}

#[doc = r#"Unique identifier for a vehicle. This will differ depending on the type of vehicle. Air platforms will have a Tail Number. Space assets will have a Sat ID. Ships and Subs will have AIS Numbers."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct VehicleUniqueIdentifierTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Ais: Option<crate::v5_0::types::AisType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TailNumber: Option<crate::v5_0::common::TailNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Satellite: Option<crate::v5_0::types::SatelliteIdentifierType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AlternateIdentifier: Option<crate::v5_0::common::AlphanumericDashSpaceUnderscoreString20Type>,
}

#[doc = r#"Video encoder output defines the multicast or file to contain the output from an encoder."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct VideoEncoderOutputTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SocketAddress: Option<crate::v5_0::choices::IpConnectionChoiceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub File: Option<crate::v5_0::types::FileNameAndOutputType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct VolumeChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpVolumeId: Option<crate::v5_0::types::OpVolumeIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VolumeTarget: Option<crate::v5_0::choices::OpVolumeType>,
}

#[doc = r#"Location associated with the specified waypoint expressed as either a geospatial or relative point."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct WayPointPointChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Point2D: Option<crate::v5_0::types::Point2DType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub RelativePoint: Option<crate::v5_0::types::Point2DRelativeType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct WeaponRestrictionTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeaponsAllowed: Option<Vec<crate::v5_0::types::StoreType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeaponsNotAllowed: Option<Vec<crate::v5_0::types::StoreType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct WeaponTargetPairingChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DmpiDesignationId: Option<crate::v5_0::types::DmpiDesignationIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DmpiPatternId: Option<crate::v5_0::types::DmpiPatternIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DmpiId: Option<Vec<crate::v5_0::types::DmpiIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Weaponeering: Option<Vec<crate::v5_0::types::WeaponeeringLocationType>>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct WeatherDataTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeatherReport: Option<crate::v5_0::types::WeatherReportDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeatherWarning: Option<crate::v5_0::types::WeatherWarningDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WeatherProductId: Option<crate::v5_0::types::ProductMetadataIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct WeatherRadarCommandTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Capability: Option<crate::v5_0::types::WeatherRadarCapabilityCommandType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Activity: Option<crate::v5_0::types::RadarActivityCommandType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct WeatherReportTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AreaData: Option<crate::v5_0::types::WeatherAreaDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub GridData: Option<Vec<crate::v5_0::types::WeatherReportGridDataType>>,
}

#[doc = r#"Indicates wind as a velocity or as a magnitude and speed value."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct WindDataChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WindVelocity: Option<crate::v5_0::types::Velocity2DType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WindMagnitude: Option<crate::v5_0::types::WindMagnitudeType>,
}

#[doc = r#"Contains the information."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct WorkingEobSourceIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WorkingEobId: Option<crate::v5_0::types::WorkingEobIdType>,
}

#[doc = r#"Contains the information for the source of a WorkingSOB."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct WorkingSobSourceIdChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SystemId: Option<crate::v5_0::types::SystemIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub WorkingSobId: Option<crate::v5_0::types::WorkingSobIdType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ZChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Z: Option<crate::v5_0::common::DistanceOffsetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AltitudeOffset: Option<crate::v5_0::types::AltitudeOffsetReferenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AbsoluteAltitude: Option<crate::v5_0::types::AltitudeReferenceType>,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct ZoneChoiceTypeSerde {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OpZoneId: Option<crate::v5_0::types::OpZoneIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ZoneTarget: Option<crate::v5_0::types::ZoneExternalType>,
}

