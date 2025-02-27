use pgrx::*;
use serde::Deserialize;

#[derive(PostgresEnum, Copy, Clone, Eq, PartialEq, Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Algorithm {
    linear,
    xgboost,
    xgboost_random_forest,
    svm,
    lasso,
    elastic_net,
    ridge,
    kmeans,
    dbscan,
    knn,
    random_forest,
    least_angle,
    lasso_least_angle,
    orthogonal_matching_pursuit,
    bayesian_ridge,
    automatic_relevance_determination,
    stochastic_gradient_descent,
    perceptron,
    passive_aggressive,
    ransac,
    theil_sen,
    huber,
    quantile,
    kernel_ridge,
    gaussian_process,
    nu_svm,
    ada_boost,
    bagging,
    extra_trees,
    gradient_boosting_trees,
    hist_gradient_boosting,
    linear_svm,
    lightgbm,
    transformers,
}

impl std::str::FromStr for Algorithm {
    type Err = ();

    fn from_str(input: &str) -> Result<Algorithm, Self::Err> {
        match input {
            "linear" => Ok(Algorithm::linear),
            "xgboost" => Ok(Algorithm::xgboost),
            "xgboost_random_forest" => Ok(Algorithm::xgboost_random_forest),
            "svm" => Ok(Algorithm::svm),
            "lasso" => Ok(Algorithm::lasso),
            "elastic_net" => Ok(Algorithm::elastic_net),
            "ridge" => Ok(Algorithm::ridge),
            "kmeans" => Ok(Algorithm::kmeans),
            "dbscan" => Ok(Algorithm::dbscan),
            "knn" => Ok(Algorithm::knn),
            "random_forest" => Ok(Algorithm::random_forest),
            "least_angle" => Ok(Algorithm::least_angle),
            "lasso_least_angle" => Ok(Algorithm::lasso_least_angle),
            "orthogonal_matching_pursuit" => Ok(Algorithm::orthogonal_matching_pursuit),
            "bayesian_ridge" => Ok(Algorithm::bayesian_ridge),
            "automatic_relevance_determination" => Ok(Algorithm::automatic_relevance_determination),
            "stochastic_gradient_descent" => Ok(Algorithm::stochastic_gradient_descent),
            "perceptron" => Ok(Algorithm::perceptron),
            "passive_aggressive" => Ok(Algorithm::passive_aggressive),
            "ransac" => Ok(Algorithm::ransac),
            "theil_sen" => Ok(Algorithm::theil_sen),
            "huber" => Ok(Algorithm::huber),
            "quantile" => Ok(Algorithm::quantile),
            "kernel_ridge" => Ok(Algorithm::kernel_ridge),
            "gaussian_process" => Ok(Algorithm::gaussian_process),
            "nu_svm" => Ok(Algorithm::nu_svm),
            "ada_boost" => Ok(Algorithm::ada_boost),
            "bagging" => Ok(Algorithm::bagging),
            "extra_trees" => Ok(Algorithm::extra_trees),
            "gradient_boosting_trees" => Ok(Algorithm::gradient_boosting_trees),
            "hist_gradient_boosting" => Ok(Algorithm::hist_gradient_boosting),
            "linear_svm" => Ok(Algorithm::linear_svm),
            "lightgbm" => Ok(Algorithm::lightgbm),
            "transformers" => Ok(Algorithm::transformers),
            _ => Err(()),
        }
    }
}

impl std::string::ToString for Algorithm {
    fn to_string(&self) -> String {
        match *self {
            Algorithm::linear => "linear".to_string(),
            Algorithm::xgboost => "xgboost".to_string(),
            Algorithm::xgboost_random_forest => "xgboost_random_forest".to_string(),
            Algorithm::svm => "svm".to_string(),
            Algorithm::lasso => "lasso".to_string(),
            Algorithm::elastic_net => "elastic_net".to_string(),
            Algorithm::ridge => "ridge".to_string(),
            Algorithm::kmeans => "kmeans".to_string(),
            Algorithm::dbscan => "dbscan".to_string(),
            Algorithm::knn => "knn".to_string(),
            Algorithm::random_forest => "random_forest".to_string(),
            Algorithm::least_angle => "least_angle".to_string(),
            Algorithm::lasso_least_angle => "lasso_least_angle".to_string(),
            Algorithm::orthogonal_matching_pursuit => "orthogonal_matching_pursuit".to_string(),
            Algorithm::bayesian_ridge => "bayesian_ridge".to_string(),
            Algorithm::automatic_relevance_determination => {
                "automatic_relevance_determination".to_string()
            }
            Algorithm::stochastic_gradient_descent => "stochastic_gradient_descent".to_string(),
            Algorithm::perceptron => "perceptron".to_string(),
            Algorithm::passive_aggressive => "passive_aggressive".to_string(),
            Algorithm::ransac => "ransac".to_string(),
            Algorithm::theil_sen => "theil_sen".to_string(),
            Algorithm::huber => "huber".to_string(),
            Algorithm::quantile => "quantile".to_string(),
            Algorithm::kernel_ridge => "kernel_ridge".to_string(),
            Algorithm::gaussian_process => "gaussian_process".to_string(),
            Algorithm::nu_svm => "nu_svm".to_string(),
            Algorithm::ada_boost => "ada_boost".to_string(),
            Algorithm::bagging => "bagging".to_string(),
            Algorithm::extra_trees => "extra_trees".to_string(),
            Algorithm::gradient_boosting_trees => "gradient_boosting_trees".to_string(),
            Algorithm::hist_gradient_boosting => "hist_gradient_boosting".to_string(),
            Algorithm::linear_svm => "linear_svm".to_string(),
            Algorithm::lightgbm => "lightgbm".to_string(),
            Algorithm::transformers => "transformers".to_string(),
        }
    }
}
