import path = require("node:path");
import * as cdk from "aws-cdk-lib";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as cloudfrontOrigins from "aws-cdk-lib/aws-cloudfront-origins";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as s3_deployment from "aws-cdk-lib/aws-s3-deployment";
import type { Construct } from "constructs";

export class MainStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Web サイトホスティング 兼 データ置き場用の S3 バケットの作成
    const bucket = new s3.Bucket(this, "weather-forecast-comparison", {
      bucketName: "weather-forecast-comparison",
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
    });

    // CloudFront ディストリビューションの作成
    const distribution = new cloudfront.Distribution(this, "Distribution", {
      defaultRootObject: "index.html",
      defaultBehavior: {
        origin:
          // S3 バケットへの OAC によるアクセス制御を設定
          cloudfrontOrigins.S3BucketOrigin.withOriginAccessControl(bucket),
      },
    });

    // CloudFront Distribution の URL を出力
    new cdk.CfnOutput(this, "DistributionUrl", {
      value: `https://${distribution.distributionDomainName}`,
    });

    // S3 バケットへの Website コンテンツのデプロイ
    new s3_deployment.BucketDeployment(this, "WebsiteDeploy", {
      sources: [
        s3_deployment.Source.asset(path.join(__dirname, "../../client/dist")),
      ],
      destinationBucket: bucket,
      distribution: distribution,
      distributionPaths: ["/*"],
    });
  }
}
