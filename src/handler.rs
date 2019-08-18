use crate::slack;
use crate::{CloudWatchEvent, CodePipelineEvent, Output};
use lambda_runtime::Context;
use log::info;
use rusoto_core::Region;
use rusoto_ssm::Ssm;
use std::str::FromStr;

type Result<T> = std::result::Result<T, failure::Error>;

pub fn my_handler(e: &CloudWatchEvent, _c: &Context) -> Result<Output> {
    let region = std::env::var("AWS_DEFAULT_REGION")?;
    let slack_webhook_url_ssm_parameter_name = std::env::var("SLACK_WEBHOOK_URL_SSM_PARAMETER")?;
    let slack_webhook_url = ssm_get_parameter(&region, &slack_webhook_url_ssm_parameter_name)?;

    let detail = &e.detail;

    info!(
        "region: {}, slack_webhook_url_ssm_parameter_name: {}, pipeline: {}, state: {}",
        region, slack_webhook_url_ssm_parameter_name, detail.pipeline, detail.state
    );

    match detail.state.as_str() {
        "STARTED" => send_message(&slack_webhook_url, &region, detail, "#364fa6")?,
        "SUCCEEDED" => send_message(&slack_webhook_url, &region, detail, "#1c7105")?,
        "FAILED" => send_message(&slack_webhook_url, &region, detail, "#c14025")?,
        _ => send_message(&slack_webhook_url, &region, detail, "#ccc")?,
    }

    Ok(Output {
        message: "Ok".to_string(),
    })
}

fn ssm_get_parameter(region: &str, name: &str) -> Result<String> {
    let ssm = rusoto_ssm::SsmClient::new(
        Region::from_str(&region).expect(&format!("AWS_DEFAULT_REGION = {}", region)),
    );
    let response = ssm
        .get_parameter(rusoto_ssm::GetParameterRequest {
            name: name.to_string(),
            with_decryption: Some(true),
        })
        .sync();

    match response?.parameter {
        Some(p) => p.value.ok_or(failure::err_msg(format!(
            "SSM parameter value {} is not set in {}",
            name, region
        ))),
        None => Err(failure::err_msg(format!(
            "SSM parameter {} is not set in {}",
            name, region
        ))),
    }
}

fn codepipeline_url(region: &str, pipeline: &str, execution_id: &str) -> String {
    format!(
        "https://{region}.console.aws.amazon.com/codesuite/codepipeline/pipelines/{pipeline}/executions/{execution_id}/timeline",
        region=region,
        pipeline=pipeline,
        execution_id=execution_id
    )
}

fn send_message(
    slack_webhook_url: &str,
    region: &str,
    detail: &CodePipelineEvent,
    color: &str,
) -> Result<()> {
    slack::send_message(
        slack_webhook_url,
        &slack::Payload {
            text: Some(format!("Pipeline: {}", detail.state)),
            attachments: vec![slack::Attachment {
                text: Some(detail.execution_id.clone()),
                title: Some(detail.pipeline.clone()),
                title_link: Some(codepipeline_url(
                    region,
                    &detail.pipeline,
                    &detail.execution_id,
                )),
                color: Some(color.to_string()),
            }],
        },
    )?;
    Ok(())
}
