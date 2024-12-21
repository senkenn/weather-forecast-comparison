import path = require("node:path");
import * as cdk from "aws-cdk-lib";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as cloudfrontOrigins from "aws-cdk-lib/aws-cloudfront-origins";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as s3_deployment from "aws-cdk-lib/aws-s3-deployment";
import type { Construct } from "constructs";

export class S3Stack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Web サイトホスティング用 S3 バケットの作成
    const websiteBucket = new s3.Bucket(this, "WebsiteBucket", {
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
    });

    // CSV データ用 S3 バケットの作成
    const dataBucket = new s3.Bucket(this, "DataBucket", {
      bucketName: "weather-forecast-comparison-data-store",
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
    });

    // CloudFront ディストリビューションの作成
    const distribution = new cloudfront.Distribution(this, "Distribution", {
      defaultRootObject: "index.html",
      defaultBehavior: {
        origin:
          // S3 バケットへの OAC によるアクセス制御を設定
          cloudfrontOrigins.S3BucketOrigin.withOriginAccessControl(
            websiteBucket,
          ),
      },
      additionalBehaviors: {
        "/data/*": {
          origin:
            cloudfrontOrigins.S3BucketOrigin.withOriginAccessControl(
              dataBucket,
            ),
        },
      },
    });

    // CloudFront Distribution の URL を出力
    new cdk.CfnOutput(this, "DistributionUrl", {
      value: `https://${distribution.distributionDomainName}`,
    });

    // Web サイトの S3 バケットへのコンテンツのデプロイ
    new s3_deployment.BucketDeployment(this, "WebsiteDeploy", {
      sources: [
        s3_deployment.Source.asset(path.join(__dirname, "../../client/dist")),
      ],
      destinationBucket: websiteBucket,
      distribution: distribution,
      distributionPaths: ["/*"],
    });
  }
}
